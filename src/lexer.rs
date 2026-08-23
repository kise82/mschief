use std::{iter::Peekable, str::CharIndices};

pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

#[derive(Debug)]
pub enum Token {
    Unknown,

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

        let (_, c) = self.iter.find(|&(_, c)| !c.is_ascii_whitespace())?;
        let token = match c {
            // Operators
            '+' => Plus,
            '-' => Minus,
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
