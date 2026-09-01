use std::{iter::Peekable, mem, str::CharIndices};

pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

#[derive(Debug)]
pub enum Token<'a> {
    Unknown,

    // Identifiers & keywords
    Ident(&'a str),
    True,
    False,

    // Literals
    Integer(i64),
    Float(f64),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Less,
    LessEquals,
    Bang,
    BangEquals,
    Equals,
    EqualsEquals,
    Greater,
    GreaterEquals,

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

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            iter: input.char_indices().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        use Token::*;

        macro_rules! double_char_token {
            ($if:expr, $then:ident, $else:ident) => {
                if let Some(&(_, next)) = self.iter.peek()
                    && next == $if
                {
                    self.iter.next();
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
            'A'..='Z' | 'a'..='z' => {
                let mut end = i + c.len_utf8();
                while let Some(&(j, next)) = self.iter.peek() {
                    if next.is_ascii_alphanumeric() {
                        self.iter.next();
                    } else {
                        end = j;
                        break;
                    }
                }

                match &self.input[i..end] {
                    "true" => True,
                    "false" => False,
                    ident => Ident(ident),
                }
            }

            // Literals
            '0'..='9' => self.parse_int_or_float(i, i + c.len_utf8()),

            // Operators
            '+' => Plus,
            '-' => {
                if let Some(&(j, next)) = self.iter.peek()
                    && next.is_ascii_digit()
                {
                    self.iter.next();
                    self.parse_int_or_float(i, j + next.len_utf8())
                } else {
                    Minus
                }
            }
            '*' => Star,
            '/' => Slash,
            '<' => double_char_token!('=', LessEquals, Less),
            '!' => double_char_token!('=', BangEquals, Bang),
            '=' => double_char_token!('=', EqualsEquals, Equals),
            '>' => double_char_token!('=', GreaterEquals, Greater),

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
            _ => Unknown,
        }
    }

    // Parsing helpers

    fn skip_whitespaces_and_comments(&mut self) -> Option<<CharIndices<'_> as Iterator>::Item> {
        loop {
            let (i, c) = self.iter.find(|&(_, c)| !c.is_ascii_whitespace())?;

            if c == '/'
                && let Some(&(_, '/')) = self.iter.peek()
            {
                let _ = self.iter.find(|&(_, c)| c == '\n');
            } else {
                return Some((i, c));
            }
        }
    }

    fn parse_int_or_float(&mut self, start: usize, assumed_end: usize) -> Token<'a> {
        use std::num::IntErrorKind;

        let mut end = assumed_end;

        let mut next = '\0';
        while let Some(&(j, c)) = self.iter.peek() {
            if c.is_ascii_digit() {
                self.iter.next();
            } else {
                next = c;
                end = j;
                break;
            }
        }

        if next == '.' {
            let mut alt_iter = self.iter.clone();
            alt_iter.next();

            let mut new_end = end;
            while let Some(&(j, c)) = alt_iter.peek() {
                if c.is_ascii_digit() {
                    alt_iter.next();
                } else {
                    new_end = j;
                    break;
                }
            }

            if new_end - end > 1 {
                end = new_end;
                let _ = mem::replace(&mut self.iter, alt_iter);
                return self.input[start..end]
                    .parse::<f64>()
                    .map_or_else(|_| Token::Error(LexError::Invalid), Token::Float);
            }
        }

        self.input[start..end].parse::<i64>().map_or_else(
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

mod utils {}
