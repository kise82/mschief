use crate::error::LexError;

use core::fmt::Debug;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Token<'a> {
    Eof,

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
    Percent,
    PercentEquals,
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
    Colon,
    LParen,
    RParen,
    LSquare,
    RSquare,
    LCurly,
    RCurly,

    // Meta
    Error(LexError),
}

impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Eof => write!(f, "EOF"),

            Self::Var(var) => write!(f, "@{var}"),
            Self::Ident(ident) => write!(f, "{ident}"),
            Self::Break => write!(f, "break"),
            Self::Continue => write!(f, "continue"),
            Self::Else => write!(f, "else"),
            Self::False => write!(f, "false"),
            Self::If => write!(f, "if"),
            Self::Proc => write!(f, "proc"),
            Self::Return => write!(f, "return"),
            Self::True => write!(f, "true"),
            Self::While => write!(f, "while"),

            Self::String(str) => write!(f, "'{str}'"),
            Self::Integer(int) => write!(f, "{int}"),
            Self::Float(float) => write!(f, "{float}"),

            Self::Plus => write!(f, "+"),
            Self::PlusEquals => write!(f, "+="),
            Self::Minus => write!(f, "-"),
            Self::MinusEquals => write!(f, "-="),
            Self::Star => write!(f, "*"),
            Self::StarEquals => write!(f, "*="),
            Self::Slash => write!(f, "/"),
            Self::SlashEquals => write!(f, "/="),
            Self::Percent => write!(f, "%"),
            Self::PercentEquals => write!(f, "%="),
            Self::Dot => write!(f, "."),
            Self::DotEquals => write!(f, ".="),
            Self::Equals => write!(f, "="),
            Self::EqualsEquals => write!(f, "=="),
            Self::Bang => write!(f, "!"),
            Self::BangEquals => write!(f, "!="),
            Self::Less => write!(f, "<"),
            Self::LessEquals => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEquals => write!(f, ">="),
            Self::AmpAmp => write!(f, "&&"),
            Self::PipePipe => write!(f, "||"),

            Self::Comma => write!(f, ","),
            Self::Semicolon => write!(f, ";"),
            Self::Colon => write!(f, ":"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LSquare => write!(f, "["),
            Self::RSquare => write!(f, "]"),
            Self::LCurly => write!(f, "{{"),
            Self::RCurly => write!(f, "}}"),

            Self::Error(err) => f.debug_tuple("ERROR").field(err).finish(),
        }
    }
}
