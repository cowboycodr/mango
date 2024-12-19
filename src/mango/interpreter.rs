use super::expression::{Expression, Visitor};
use super::literal::{Literal, Fac, Pow};
use super::token_type::TokenType;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expression: Expression) -> Literal {
        expression.accept(self)
    }
}

impl Visitor<Literal> for Interpreter {
    fn visit_binary(&mut self, left: &Expression, operator: &super::token::Token, right: &Expression) -> Literal {
        let left = left.accept(self);
        let right = right.accept(self);

        match operator.kind {
            TokenType::StarStar => left.pow(right),
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Plus => left + right,
            TokenType::Minus => left - right,

            _ => panic!("Unsupported binary operator")
        }
    }
    
    fn visit_unary(&mut self, operator: &super::token::Token, right: &Expression, is_prefix: &bool) -> Literal {
        let right = right.accept(self);

        match (operator.kind, is_prefix) {
            (TokenType::Bang, true) => !right,
            (TokenType::Minus, true) => -right,

            (TokenType::Bang, false) => right.fac(),

            _ => panic!("Unsupported unary operator"),
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> Literal {
        literal.clone()
    }

    fn visit_grouping(&mut self, expression: &Expression) -> Literal {
        expression.accept(self)
    }
}