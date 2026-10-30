use crate::error::LexError;

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
