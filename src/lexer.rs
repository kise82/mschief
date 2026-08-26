use std::{iter::Peekable, str::CharIndices};

pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

#[derive(Debug)]
pub enum Token {
    Unknown,

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
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Error
    Error(LexError),
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
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

        let (i, c) = self.iter.find(|&(_, c)| !c.is_ascii_whitespace())?;
        let token = match c {
            // Literals
            '0'..='9' => utils::parse_int_or_float((i, i + c.len_utf8()), self),

            // Operators
            '+' => Plus,
            '-' => {
                if let Some(&(j, next)) = self.iter.peek()
                    && next.is_ascii_digit()
                {
                    self.iter.next();
                    utils::parse_int_or_float((i, j + next.len_utf8()), self)
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
            '{' => LBrace,
            '}' => RBrace,
            '[' => LBracket,
            ']' => RBracket,

            // Rest
            _ => Unknown,
        };
        Some(token)
    }
}

mod utils {
    use super::{LexError, Lexer, Token};
    use std::{mem, num::IntErrorKind};

    pub fn parse_int_or_float(initial_bounds: (usize, usize), lexer: &mut Lexer) -> Token {
        let iter = &mut lexer.iter;

        let start = initial_bounds.0;
        let mut end = initial_bounds.1;

        let mut next = '\0';
        while let Some(&(j, c)) = iter.peek() {
            if c.is_ascii_digit() {
                iter.next();
            } else {
                next = c;
                end = j;
                break;
            }
        }

        if next == '.' {
            let mut alt_iter = iter.clone();
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
                let _ = mem::replace(&mut lexer.iter, alt_iter);
                return lexer.input[start..end]
                    .parse::<f64>()
                    .map_or_else(|_| Token::Error(LexError::Invalid), Token::Float);
            }
        }

        lexer.input[start..end].parse::<i64>().map_or_else(
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
