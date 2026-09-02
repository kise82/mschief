use std::{iter::Peekable, mem, str::CharIndices};

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
                if let Some(&(_, $if)) = self.iter.peek() {
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
                let &(j, _) =
                    utils::next_while(&mut self.iter, |&(_, c)| c.is_ascii_alphanumeric())
                        .unwrap_or(&(self.input.len(), '\0'));

                match &self.input[i..j] {
                    "true" => True,
                    "false" => False,
                    ident => Ident(ident),
                }
            }

            // Literals
            '0'..='9' => self.parse_int_or_float(i),

            // Operators
            '+' => Plus,
            '-' => {
                if let Some(&(_, '0'..='9')) = self.iter.peek() {
                    self.iter.next();
                    self.parse_int_or_float(i)
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

    #[inline(always)]
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
