use core::iter::Peekable;

use bumpalo::boxed::Box;

use crate::driver::Driver;
use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq)]
pub enum AstNode<'a> {
    Error,
    Literal(Token<'a>),
    Unary {
        op: Token<'a>,
        operand: Box<'a, AstNode<'a>>,
    },
    Binary {
        op: Token<'a>,
        lhs: Box<'a, AstNode<'a>>,
        rhs: Box<'a, AstNode<'a>>,
    },
}

impl<'a> AstNode<'a> {
    pub fn new_literal(driver: &'a Driver<'a>, value: Token<'a>) -> Box<'a, Self> {
        driver.alloc(Self::Literal(value))
    }

    pub fn new_unary(
        driver: &'a Driver<'a>,
        op: Token<'a>,
        operand: Box<'a, Self>,
    ) -> Box<'a, Self> {
        driver.alloc(Self::Unary { op, operand })
    }

    pub fn new_binary(
        driver: &'a Driver<'a>,
        op: Token<'a>,
        lhs: Box<'a, Self>,
        rhs: Box<'a, Self>,
    ) -> Box<'a, Self> {
        driver.alloc(Self::Binary { op, lhs, rhs })
    }

    pub fn new_error(driver: &'a Driver<'a>) -> Box<'a, Self> {
        driver.alloc(Self::Error)
    }
}

#[inline(always)]
fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
    match token {
        Token::AmpAmp | Token::PipePipe => Some((1, 2)),
        Token::EqualsEquals | Token::BangEquals => Some((3, 4)),
        Token::Less | Token::LessEquals | Token::Greater | Token::GreaterEquals => Some((5, 6)),
        Token::Plus | Token::Minus => Some((7, 8)),
        Token::Star | Token::Slash => Some((9, 10)),
        _ => None,
    }
}

#[inline(always)]
fn prefix_binding_power(token: &Token) -> Option<u8> {
    match token {
        Token::Plus | Token::Minus => Some(11),
        _ => None,
    }
}

pub struct Parser<'a> {
    driver: &'a Driver<'a>,
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(driver: &'a Driver<'a>, input: &'a str) -> Self {
        Self {
            driver,
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn from_lexer(driver: &'a Driver<'a>, lexer: Lexer<'a>) -> Self {
        Self {
            driver,
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> Box<'a, AstNode<'a>> {
        self.program()
    }

    // Rules

    #[inline(always)]
    pub fn program(&mut self) -> Box<'a, AstNode<'a>> {
        self.expression()
    }

    #[inline(always)]
    pub fn expression(&mut self) -> Box<'a, AstNode<'a>> {
        self.parse_expr(0)
    }

    pub fn parse_expr(&mut self, min_bp: u8) -> Box<'a, AstNode<'a>> {
        let mut lhs = match self.lexer.next() {
            Some(token) => match token {
                value @ Token::Integer(_)
                | value @ Token::Float(_)
                | value @ Token::Var(_)
                | value @ Token::True
                | value @ Token::False
                | value @ Token::String(_) => AstNode::new_literal(self.driver, value),

                Token::LParen => {
                    let inner = self.parse_expr(0);
                    if let Some(Token::RParen) = self.lexer.next() {
                        inner
                    } else {
                        AstNode::new_error(self.driver)
                    }
                }

                op @ Token::Plus | op @ Token::Minus => {
                    let rbp = prefix_binding_power(&op).unwrap_or(11);
                    let operand = self.parse_expr(rbp);
                    AstNode::new_unary(self.driver, op, operand)
                }

                _ => AstNode::new_error(self.driver),
            },
            None => AstNode::new_error(self.driver),
        };

        if matches!(*lhs, AstNode::Error) {
            return lhs;
        }

        while let Some(op) = self.lexer.peek() {
            let rbp = match infix_binding_power(op) {
                Some((lbp, rbp)) => {
                    if lbp < min_bp {
                        break;
                    }
                    rbp
                }
                None => break,
            };

            let op_token = self.lexer.next().unwrap();
            let rhs = self.parse_expr(rbp);
            let rhs_is_error = matches!(*rhs, AstNode::Error);
            lhs = AstNode::new_binary(self.driver, op_token, lhs, rhs);
            if rhs_is_error {
                break;
            }
        }

        lhs
    }
}
