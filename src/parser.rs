use core::iter::Peekable;

use bumpalo::boxed::Box;

use crate::driver::Driver;
use crate::error::ParseError;
use crate::lexer::Lexer;
use crate::tokens::Token;

#[derive(Debug, PartialEq)]
pub enum AstNode<'a> {
    Error(ParseError<'a>),
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

    pub fn new_error(driver: &'a Driver<'a>, error: ParseError<'a>) -> Box<'a, Self> {
        driver.alloc(Self::Error(error))
    }

    pub fn first_error(&self) -> Option<&ParseError<'a>> {
        match self {
            Self::Error(err) => Some(err),
            Self::Unary { operand, .. } => operand.first_error(),
            Self::Binary { lhs, rhs, .. } => lhs.first_error().or_else(|| rhs.first_error()),
            Self::Literal(_) => None,
        }
    }

    pub fn has_error(&self) -> bool {
        self.first_error().is_some()
    }

    pub fn for_each_error<F>(&self, mut f: F)
    where
        F: FnMut(&ParseError<'a>),
    {
        self.walk_errors(&mut f);
    }

    fn walk_errors<F>(&self, f: &mut F)
    where
        F: FnMut(&ParseError<'a>),
    {
        match self {
            Self::Error(err) => f(err),
            Self::Literal(_) => {}
            Self::Unary { operand, .. } => operand.walk_errors(f),
            Self::Binary { lhs, rhs, .. } => {
                lhs.walk_errors(f);
                rhs.walk_errors(f);
            }
        }
    }
}

pub struct Parser<'a> {
    driver: &'a Driver<'a>,
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` to process `input` in behalf of `driver`.
    pub fn new(driver: &'a Driver<'a>, input: &'a str) -> Self {
        Self {
            driver,
            lexer: Lexer::new(input).peekable(),
        }
    }

    /// Returns the AST for the input.
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
                    if inner.has_error() {
                        if let Some(Token::RParen) = self.lexer.peek() {
                            let _ = self.lexer.next();
                        }
                        inner
                    } else {
                        match self.lexer.next() {
                            Some(Token::RParen) => inner,
                            other => AstNode::new_error(
                                self.driver,
                                ParseError::unexpected(")", other.unwrap_or(Token::Eof)),
                            ),
                        }
                    }
                }

                op @ Token::Plus | op @ Token::Minus => {
                    let rbp = attrs::prefix_binding_power(&op).unwrap_or(13);
                    let operand = self.parse_expr(rbp);
                    AstNode::new_unary(self.driver, op, operand)
                }

                other => {
                    AstNode::new_error(self.driver, ParseError::unexpected("expression", other))
                }
            },
            None => AstNode::new_error(
                self.driver,
                ParseError::unexpected("expression", Token::Eof),
            ),
        };

        if lhs.has_error() {
            return lhs;
        }

        while let Some(op) = self.lexer.peek() {
            let rbp = match attrs::infix_binding_power(op) {
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
            let rhs_is_error = rhs.has_error();
            lhs = AstNode::new_binary(self.driver, op_token, lhs, rhs);
            if rhs_is_error {
                break;
            }
        }

        lhs
    }
}

mod attrs {
    use super::Token;

    #[inline(always)]
    pub fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
        match token {
            Token::PipePipe => Some((1, 2)),
            Token::AmpAmp => Some((3, 4)),
            Token::EqualsEquals | Token::BangEquals => Some((5, 6)),
            Token::Less | Token::LessEquals | Token::Greater | Token::GreaterEquals => Some((7, 8)),
            Token::Plus | Token::Minus => Some((9, 10)),
            Token::Star | Token::Slash | Token::Percent => Some((11, 12)),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn prefix_binding_power(token: &Token) -> Option<u8> {
        match token {
            Token::Plus | Token::Minus => Some(13),
            _ => None,
        }
    }
}
