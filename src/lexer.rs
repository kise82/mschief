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
            '0'..='9' => utils::parse_int((i, c), self.input, &mut self.iter),

            // Operators
            '+' => Plus,
            '-' => {
                if let Some(&(_, next)) = self.iter.peek()
                    && next.is_ascii_digit()
                {
                    self.iter.next();
                    utils::parse_int((i, next), self.input, &mut self.iter)
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
    use super::{LexError, Token};
    use std::{iter::Peekable, num::IntErrorKind, str::CharIndices};

    pub fn parse_int<'a>(
        start: (usize, char),
        input: &'a str,
        iter: &mut Peekable<CharIndices<'a>>,
    ) -> Token {
        let mut end = start.0 + start.1.len_utf8();
        while let Some(&(j, next)) = iter.peek() {
            if next.is_ascii_digit() {
                iter.next();
            } else {
                end = j;
                break;
            }
        }

        input[start.0..end].parse::<i64>().map_or_else(
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
