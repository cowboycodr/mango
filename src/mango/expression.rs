use super::token::Token;
use super::token_type::TokenType;

use super::literal::{Fac, Literal, Pow};

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
        is_prefix: bool,
    },
    Literal(Literal),
    Grouping {
        expression: Box<Expression>,
    },
}

impl Expression {
    pub fn evaluate(&self) -> Literal {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => match operator.kind {
                TokenType::Plus => left.evaluate() + right.evaluate(),
                TokenType::Minus => left.evaluate() - right.evaluate(),
                TokenType::Star => left.evaluate() * right.evaluate(),
                TokenType::Slash => left.evaluate() / right.evaluate(),

                TokenType::StarStar => left.evaluate().pow(right.evaluate()),
                kind => panic!("Unexpected binary operator: {:?}", kind),
            },
            Expression::Unary {
                operator,
                right,
                is_prefix,
            } => match operator.kind {
                TokenType::Minus => -right.evaluate(),
                TokenType::Bang => {
                    if *is_prefix {
                        !right.evaluate()
                    } else {
                        right.evaluate().fac()
                    }
                }
                kind => panic!("Unexpected unary operator {:?}", kind),
            },
            Expression::Literal(literal) => literal.clone(),
            Expression::Grouping { expression } => expression.evaluate(),
        }
    }
}
