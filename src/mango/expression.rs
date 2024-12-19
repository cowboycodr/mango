use super::literal::Literal;
use super::token::Token;

// use super::token_type::TokenType;

// use super::literal::{Fac, Literal, Pow};

// #[derive(Debug)]
// pub enum Expression {
//     Binary {
//         left: Box<Expression>,
//         operator: Token,
//         right: Box<Expression>,
//     },
//     Unary {
//         operator: Token,
//         right: Box<Expression>,
//         is_prefix: bool,
//     },
//     Literal(Literal),
//     Grouping {
//         expression: Box<Expression>,
//     },
// }

// impl Expression {
//     pub fn evaluate(&self) -> Literal {
//         match self {
//             Expression::Binary {
//                 left,
//                 operator,
//                 right,
//             } => match operator.kind {
//                 TokenType::Plus => left.evaluate() + right.evaluate(),
//                 TokenType::Minus => left.evaluate() - right.evaluate(),
//                 TokenType::Star => left.evaluate() * right.evaluate(),
//                 TokenType::Slash => left.evaluate() / right.evaluate(),

//                 TokenType::StarStar => left.evaluate().pow(right.evaluate()),
//                 kind => panic!("Unexpected binary operator: {:?}", kind),
//             },
//             Expression::Unary {
//                 operator,
//                 right,
//                 is_prefix,
//             } => match operator.kind {
//                 TokenType::Minus => -right.evaluate(),
//                 TokenType::Bang => {
//                     if *is_prefix {
//                         !right.evaluate()
//                     } else {
//                         right.evaluate().fac()
//                     }
//                 }
//                 kind => panic!("Unexpected unary operator {:?}", kind),
//             },
//             Expression::Literal(literal) => literal.clone(),
//             Expression::Grouping { expression } => expression.evaluate(),
//         }
//     }
// }

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
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expression::Unary {
                operator,
                right,
                is_prefix,
            } => visitor.visit_unary(operator, right, is_prefix),
            Expression::Literal(literal) => visitor.visit_literal(literal),
            Expression::Grouping { expression } => visitor.visit_grouping(expression),
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary(&mut self, left: &Expression, operator: &Token, right: &Expression) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &Expression, is_prefix: &bool) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_grouping(&mut self, expression: &Expression) -> T;
}
