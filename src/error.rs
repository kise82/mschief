use core::fmt::Display;

use crate::tokens::Token;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LexError {
    line: usize,
    kind: LexErrorKind,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LexErrorKind {
    IntOverflow,
    UnexpectedChar(char),
    UnterminatedString,
}

impl LexError {
    pub fn new(line: usize, kind: LexErrorKind) -> Self {
        Self { line, kind }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError<'a> {
    Lex(LexError),
    UnexpectedToken {
        expected: &'static str,
        found: Token<'a>,
    },
}

impl<'a> ParseError<'a> {
    pub fn unexpected(expected: &'static str, found: Token<'a>) -> Self {
        match found {
            Token::Error(err) => Self::Lex(err),
            other => Self::UnexpectedToken {
                expected,
                found: other,
            },
        }
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let LexError { line, kind } = self;
        let _ = write!(f, "[Line {line}] ");
        match kind {
            LexErrorKind::IntOverflow => write!(f, "Integer literal overflows."),
            LexErrorKind::UnexpectedChar(c) => write!(f, "Unexpected character `{c}`."),
            LexErrorKind::UnterminatedString => {
                write!(f, "String literal started but never terminated.")
            }
        }
    }
}
