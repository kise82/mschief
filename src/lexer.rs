use std::{fmt::Debug, iter::Peekable, mem, str::CharIndices};

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
    Integer(i64),
    Float(f64),

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
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            iter: input.char_indices().peekable(),
        }
    }

    // Lexing

    pub fn next_token(&mut self) -> Token<'a> {
        use Token::*;

        macro_rules! double_char_token {
            ($if:expr, $then:ident, $else:ident) => {
                if self.iter.next_if(|&(_, c)| c == $if).is_some() {
                    $then
                } else {
                    $else
                }
            };
        }

        let Some((i, c)) = self.skip_whitespaces_and_comments() else {
            return Eof;
        };

        match c {
            // Identifiers & keywords
            '@' if let Some((i, _)) =
                self.iter.next_if(|&(_, c)| c.is_alphanumeric() || c == '_') =>
            {
                self.parse_var_or_ident(i, true)
            }
            'A'..='Z' | 'a'..='z' | '_' => self.parse_var_or_ident(i, false),

            // Literals
            '\'' | '"' => self.parse_string(i + c.len_utf8(), c),
            '0'..='9' => self.parse_int_or_float(i),

            // Operators
            '+' => double_char_token!('=', PlusEquals, Plus),
            '-' => match self.iter.next_if(|&(_, c)| c == '=') {
                Some(_) => MinusEquals,
                None if self.iter.next_if(|&(_, c)| c.is_ascii_digit()).is_some() => {
                    self.parse_int_or_float(i)
                }
                None => Minus,
            },
            '*' => double_char_token!('=', StarEquals, Star),
            '/' => double_char_token!('=', SlashEquals, Slash),
            '.' => double_char_token!('=', DotEquals, Dot),
            '=' => double_char_token!('=', EqualsEquals, Equals),
            '!' => double_char_token!('=', BangEquals, Bang),
            '<' => double_char_token!('=', LessEquals, Less),
            '>' => double_char_token!('=', GreaterEquals, Greater),
            '&' if self.iter.next_if(|&(_, c)| c == '&').is_some() => AmpAmp,
            '|' if self.iter.next_if(|&(_, c)| c == '|').is_some() => PipePipe,

            // Markers
            ',' => Comma,
            ';' => Semicolon,
            '(' => LParen,
            ')' => RParen,
            '[' => LSquare,
            ']' => RSquare,
            '{' => LCurly,
            '}' => RCurly,

            // Rest
            _ => Unknown(c),
        }
    }

    // Parsing helpers

    #[inline(always)]
    fn skip_whitespaces_and_comments(&mut self) -> Option<<CharIndices<'_> as Iterator>::Item> {
        loop {
            let (i, c) = self.iter.find(|&(_, c)| !c.is_ascii_whitespace())?;

            match c {
                '#' => {
                    let _ = self.iter.find(|&(_, c)| c == '\n');
                }
                '/' if self.iter.next_if(|&(_, c)| c == '/').is_some() => {
                    let _ = self.iter.find(|&(_, c)| c == '\n');
                }
                '/' if self.iter.next_if(|&(_, c)| c == '*').is_some() => loop {
                    let (_, _) = self.iter.find(|&(_, c)| c == '*')?;
                    if self.iter.next_if(|&(_, c)| c == '/').is_some() {
                        break;
                    }
                },
                _ => return Some((i, c)),
            }
        }
    }

    #[inline(always)]
    fn parse_var_or_ident(&mut self, start: usize, is_var: bool) -> Token<'a> {
        let &(j, _) = utils::next_while(&mut self.iter, |&(_, c)| c.is_alphanumeric() || c == '_')
            .unwrap_or(&(self.input.len(), '\0'));

        match &self.input[start..j] {
            // Variables - anything goes
            var if is_var => Token::Var(var),

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
            ident => Token::Ident(ident),
        }
    }

    #[inline(always)]
    fn parse_string(&mut self, start: usize, delimiter: char) -> Token<'a> {
        if let Some((end, _)) = self.iter.find(|&(_, c)| c == delimiter) {
            Token::String(&self.input[start..end])
        } else {
            Token::Error(LexError::Invalid)
        }
    }

    #[inline(always)]
    fn parse_int_or_float(&mut self, start: usize) -> Token<'a> {
        use std::num::IntErrorKind;

        let &(i, c) = utils::next_while(&mut self.iter, |&(_, c)| c.is_ascii_digit())
            .unwrap_or(&(self.input.len(), '\0'));

        if c == '.' {
            let mut alt_iter = self.iter.clone();
            alt_iter.next();

            let &(j, _) = utils::next_while(&mut alt_iter, |&(_, c)| c.is_ascii_digit())
                .unwrap_or(&(self.input.len(), '\0'));

            if j - i > 1 {
                let _ = mem::replace(&mut self.iter, alt_iter);
                return self.input[start..j]
                    .parse::<f64>()
                    .map_or_else(|_| Token::Error(LexError::Invalid), Token::Float);
            }
        }

        self.input[start..i].parse::<i64>().map_or_else(
            |err| {
                let kind = match err.kind() {
                    IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => LexError::IntOverflow,
                    _ => LexError::Invalid,
                };
                Token::Error(kind)
            },
            Token::Integer,
        )
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

mod utils {
    use std::iter::Peekable;

    #[inline(always)]
    pub fn next_while<I: Iterator>(
        iter: &mut Peekable<I>,
        mut predicate: impl FnMut(&I::Item) -> bool,
    ) -> Option<&I::Item> {
        while iter.next_if(&mut predicate).is_some() {}
        iter.peek()
    }
}
