use std::iter::Peekable;

use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum AstNode<'a> {
    Error,
    Literal(Token<'a>),
    Unary {
        op: Token<'a>,
        operand: Box<AstNode<'a>>,
    },
    Binary {
        op: Token<'a>,
        lhs: Box<AstNode<'a>>,
        rhs: Box<AstNode<'a>>,
    },
}

impl<'a> AstNode<'a> {
    pub fn new_literal(value: Token<'a>) -> Box<Self> {
        Box::new(Self::Literal(value))
    }

    pub fn new_unary(op: Token<'a>, operand: Box<Self>) -> Box<Self> {
        Box::new(Self::Unary { op, operand })
    }

    pub fn new_binary(op: Token<'a>, lhs: Box<Self>, rhs: Box<Self>) -> Box<Self> {
        Box::new(Self::Binary { op, lhs, rhs })
    }
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Box<AstNode<'a>> {
        self.program()
    }

    // Rules

    #[inline(always)]
    fn program(&mut self) -> Box<AstNode<'a>> {
        self.expression()
    }

    #[inline(always)]
    fn expression(&mut self) -> Box<AstNode<'a>> {
        self.logical_disjunction()
    }

    #[inline(always)]
    fn logical_disjunction(&mut self) -> Box<AstNode<'a>> {
        let mut ret = self.logical_conjunction();
        while self.lexer.next_if(|t| *t == Token::PipePipe).is_some() {
            ret = AstNode::new_binary(Token::PipePipe, ret, self.logical_conjunction());
        }
        ret
    }

    #[inline(always)]
    fn logical_conjunction(&mut self) -> Box<AstNode<'a>> {
        let mut ret = self.relational();
        while self.lexer.next_if(|t| *t == Token::AmpAmp).is_some() {
            ret = AstNode::new_binary(Token::AmpAmp, ret, self.relational());
        }
        ret
    }

    #[inline(always)]
    fn relational(&mut self) -> Box<AstNode<'a>> {
        let mut ret = self.term();
        if let Some(op) = self.lexer.next_if(|t| {
            matches!(
                *t,
                Token::EqualsEquals
                    | Token::BangEquals
                    | Token::Less
                    | Token::LessEquals
                    | Token::Greater
                    | Token::GreaterEquals
            )
        }) {
            ret = AstNode::new_binary(op, ret, self.term());
        }
        ret
    }

    #[inline(always)]
    fn term(&mut self) -> Box<AstNode<'a>> {
        let mut ret = self.factor();
        while let Some(op) = self
            .lexer
            .next_if(|t| matches!(*t, Token::Plus | Token::Minus))
        {
            ret = AstNode::new_binary(op, ret, self.factor());
        }
        ret
    }

    #[inline(always)]
    fn factor(&mut self) -> Box<AstNode<'a>> {
        let mut ret = self.atom();
        while let Some(op) = self
            .lexer
            .next_if(|t| matches!(*t, Token::Star | Token::Slash))
        {
            ret = AstNode::new_binary(op, ret, self.atom());
        }
        ret
    }

    #[inline(always)]
    fn atom(&mut self) -> Box<AstNode<'a>> {
        if let Some(token) = self.lexer.next() {
            match token {
                value @ Token::Integer(_)
                | value @ Token::Float(_)
                | value @ Token::True
                | value @ Token::False => AstNode::new_literal(value),
                Token::LParen => {
                    let inner = self.expression();
                    if self.lexer.next_if(|t| *t == Token::RParen).is_some() {
                        inner
                    } else {
                        Box::new(AstNode::Error)
                    }
                }
                _ => Box::new(AstNode::Error),
            }
        } else {
            Box::new(AstNode::Error)
        }
    }
}
