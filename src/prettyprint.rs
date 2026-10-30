use core::fmt::Debug;

use crate::{parser::AstNode, tokens::Token};

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

impl<'a> Debug for AstNode<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Error(err) => write!(f, "(error {err:?})"),
            Self::Literal(inner) => write!(f, "{inner:?}"),
            Self::Unary { op, operand } => write!(f, "({op:?} {operand:?})"),
            Self::Binary { op, lhs, rhs } => write!(f, "({op:?} {lhs:?} {rhs:?})"),
        }
    }
}
