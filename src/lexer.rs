use core::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Unknown(char),

    // Identifiers & keywords
    Var(&'a str),
    Ident(&'a str),

    Break,
    Continue,
    Else,
    False,
    If,
    Proc,
    Return,
    True,
    While,

    // Literals
    String(&'a str),
    Integer(&'a str),
    Float(&'a str),

    // Operators
    Plus,
    PlusEquals,
    Minus,
    MinusEquals,
    Star,
    StarEquals,
    Slash,
    SlashEquals,
    Dot,
    DotEquals,
    Equals,
    EqualsEquals,
    Bang,
    BangEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    AmpAmp,
    PipePipe,

    // Markers
    Comma,
    Semicolon,
    LParen,
    RParen,
    LSquare,
    RSquare,
    LCurly,
    RCurly,

    // Meta
    Error(LexError),
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    Invalid,
    IntOverflow,
}

pub struct Lexer<'a> {
    input: &'a str,
    bytes: &'a [u8],
    len: usize,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        Self {
            input,
            bytes,
            len: bytes.len(),
            cursor: 0,
        }
    }

    // Lexing

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
                if (self.cursor + 1) < self.len && self.bytes[self.cursor + 1] == $if {
                    self.cursor += 2;
                    $then
                } else {
                    self.cursor += 1;
                    $else
                }
            }};
        }

        let c = self.skip_whitespaces_and_comments();

        let start = self.cursor;
        let bytes = self.bytes;
        let len = self.len;
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
            b'\'' | b'"' => {
                cur += 1;
                while cur < len && bytes[cur] != c {
                    cur += 1;
                }

                if cur < len {
                    self.cursor = cur + 1;
                    String(&self.input[start + 1..cur])
                } else {
                    self.cursor = cur;
                    Error(LexError::Invalid)
                }
            }

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
        let len = self.len;
        let mut cur = self.cursor;
        loop {
            while cur < len && bytes[cur].is_ascii_whitespace() {
                cur += 1;
            }

            if cur >= len {
                self.cursor = len;
                return b'\0';
            }

            match bytes[cur] {
                b'#' => {
                    cur += 1;
                    while cur < len && bytes[cur] != b'\n' {
                        cur += 1;
                    }
                }
                b'/' => {
                    if (cur + 1) < len {
                        match bytes[cur + 1] {
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
                                    }
                                    cur += 1;
                                }
                            }
                            _ => {
                                self.cursor = cur;
                                return b'/';
                            }
                        }
                    } else {
                        self.cursor = cur;
                        return b'/';
                    }
                }
                c => {
                    self.cursor = cur;
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
