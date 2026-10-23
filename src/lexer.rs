pub use crate::tokens::{LexError, Token};

pub struct Lexer<'a> {
    input: &'a str,
    bytes: &'a [u8],
    cursor: usize,

    line: usize,
    next_line: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` to process `input`.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            cursor: 0,

            line: 1,
            next_line: 1,
        }
    }

    /// Returns the line in which the last `Token` started.
    pub const fn line(&self) -> usize {
        self.line
    }

    // Lexing

    /// Lex the next available `Token` of the input, if any.
    pub fn next_token(&mut self) -> Token<'a> {
        use Token::*;

        macro_rules! single_char_token {
            ($tok:ident) => {{
                self.cursor += 1;
                $tok
            }};
        }

        macro_rules! double_char_token {
            ($if:expr, $then:ident, $else:ident) => {{
                if (self.cursor + 1) < self.bytes.len() && self.bytes[self.cursor + 1] == $if {
                    self.cursor += 2;
                    $then
                } else {
                    self.cursor += 1;
                    $else
                }
            }};
        }

        let c = self.skip_whitespaces_and_comments();

        self.line = self.next_line;

        let start = self.cursor;
        let bytes = self.bytes;
        let len = bytes.len();
        let mut cur = start;

        match c {
            // Identifiers & keywords
            b'@' => {
                cur += 1;
                while cur < len && (bytes[cur].is_ascii_alphanumeric() || bytes[cur] == b'_') {
                    cur += 1;
                }

                self.cursor = cur;

                if (cur - start) > 1 {
                    Var(&self.input[start + 1..cur])
                } else {
                    Unknown(c as char)
                }
            }

            b'A'..=b'Z' | b'a'..=b'z' | b'_' => {
                while cur < len && (bytes[cur].is_ascii_alphabetic() || bytes[cur] == b'_') {
                    cur += 1;
                }

                self.cursor = cur;

                Self::keyword_or_ident(&self.input[start..cur])
            }

            // Literals
            b'\'' | b'"' => self.string(c),

            b'0'..=b'9' => {
                while cur < len && bytes[cur].is_ascii_digit() {
                    cur += 1;
                }

                if (cur + 1) < len && bytes[cur] == b'.' && bytes[cur + 1].is_ascii_digit() {
                    cur += 2;
                    while cur < len && bytes[cur].is_ascii_digit() {
                        cur += 1;
                    }
                    self.cursor = cur;
                    Float(&self.input[start..cur])
                } else {
                    self.cursor = cur;
                    Integer(&self.input[start..cur])
                }
            }

            // Operators
            b'+' => double_char_token!(b'=', PlusEquals, Plus),
            b'-' => double_char_token!(b'=', MinusEquals, Minus),
            b'*' => double_char_token!(b'=', StarEquals, Star),
            b'/' => double_char_token!(b'=', SlashEquals, Slash),
            b'%' => double_char_token!(b'=', PercentEquals, Percent),
            b'.' => double_char_token!(b'=', DotEquals, Dot),
            b'=' => double_char_token!(b'=', EqualsEquals, Equals),
            b'!' => double_char_token!(b'=', BangEquals, Bang),
            b'<' => double_char_token!(b'=', LessEquals, Less),
            b'>' => double_char_token!(b'=', GreaterEquals, Greater),
            b'&' if (cur + 1) < len && bytes[cur + 1] == b'&' => {
                self.cursor += 2;
                AmpAmp
            }
            b'|' if (cur + 1) < len && bytes[cur + 1] == b'|' => {
                self.cursor += 2;
                PipePipe
            }

            // Markers
            b',' => single_char_token!(Comma),
            b';' => single_char_token!(Semicolon),
            b':' => single_char_token!(Colon),
            b'(' => single_char_token!(LParen),
            b')' => single_char_token!(RParen),
            b'[' => single_char_token!(LSquare),
            b']' => single_char_token!(RSquare),
            b'{' => single_char_token!(LCurly),
            b'}' => single_char_token!(RCurly),

            b'\0' => Eof,
            _ => {
                let c = self.input[start..]
                    .chars()
                    .next()
                    .expect("There should be at least 1 `char` left");
                self.cursor += c.len_utf8();
                Unknown(c)
            }
        }
    }

    // Parsing helpers

    #[inline(always)]
    fn skip_whitespaces_and_comments(&mut self) -> u8 {
        let bytes = self.bytes;
        let len = bytes.len();
        let mut cur = self.cursor;
        let mut line = self.next_line;
        loop {
            while cur < len && bytes[cur].is_ascii_whitespace() {
                if bytes[cur] == b'\n' {
                    line += 1;
                }
                cur += 1;
            }

            if cur >= len {
                self.cursor = len;
                self.next_line = line;
                return b'\0';
            }

            match bytes[cur] {
                b'#' => {
                    cur += 1;
                    while cur < len && bytes[cur] != b'\n' {
                        cur += 1;
                    }
                }
                b'/' => match bytes.get(cur + 1).unwrap_or(&b'\0') {
                    b'/' => {
                        cur += 2;
                        while cur < len && bytes[cur] != b'\n' {
                            cur += 1;
                        }
                    }
                    b'*' => {
                        cur += 2;
                        while cur + 1 < len {
                            if bytes[cur] == b'*' && bytes[cur + 1] == b'/' {
                                cur += 2;
                                break;
                            } else if bytes[cur] == b'\n' {
                                line += 1;
                            }
                            cur += 1;
                        }
                    }
                    _ => {
                        self.cursor = cur;
                        self.next_line = line;
                        return b'/';
                    }
                },
                c => {
                    self.cursor = cur;
                    self.next_line = line;
                    return c;
                }
            }
        }
    }

    #[inline(always)]
    fn keyword_or_ident(lexeme: &'a str) -> Token<'a> {
        match lexeme {
            // Keywords
            "break" => Token::Break,
            "continue" => Token::Continue,
            "else" => Token::Else,
            "false" => Token::False,
            "if" => Token::If,
            "proc" => Token::Proc,
            "return" => Token::Return,
            "true" => Token::True,
            "while" => Token::While,

            // Identifiers - at last
            _ => Token::Ident(lexeme),
        }
    }

    #[inline(always)]
    pub fn string(&mut self, delimiter: u8) -> Token<'a> {
        let start = self.cursor;
        let bytes = self.bytes;
        let len = bytes.len();
        let mut cur = start + 1;
        let mut line = self.next_line;

        while cur < len {
            match bytes[cur] {
                b'\\' => cur += 2,
                c if c == delimiter => break,
                c => {
                    if c == b'\n' {
                        line += 1;
                    }
                    cur += 1;
                }
            }
        }

        self.next_line = line;

        if cur < len {
            self.cursor = cur + 1;
            Token::String(&self.input[start + 1..cur])
        } else {
            self.cursor = len;
            Token::Error(LexError::Invalid)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::Eof => None,
            token => Some(token),
        }
    }
}
