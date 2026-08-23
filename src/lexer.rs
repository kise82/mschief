use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
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
        let input = input.chars().peekable();
        Self { input }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        macro_rules! double_char_token {
            ($if:expr, $then:ident, $else:ident) => {
                if let Some(&next) = self.input.peek()
                    && next == $if
                {
                    self.input.next();
                    $then
                } else {
                    $else
                }
            };
        }

        let c = self.input.find(|&c| !c.is_ascii_whitespace())?;
        Some(match c {
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
        })
    }
}
