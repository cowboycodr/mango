use super::environment::Environment;
use super::expression::{self, Expression};
use super::statement::{self, Statement};

use super::literal::{Fac, Literal, Pow};
use super::token_type::TokenType;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statement: Statement) {
        statement.accept(self)
    }
}

impl expression::Visitor<Literal> for Interpreter {
    fn visit_binary(
        &mut self,
        left: &Expression,
        operator: &super::token::Token,
        right: &Expression,
    ) -> Literal {
        let left = left.accept(self);
        let right = right.accept(self);

        match operator.kind {
            TokenType::StarStar => left.pow(right),
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Plus => left + right,
            TokenType::Minus => left - right,

            _ => panic!("Unsupported binary operator"),
        }
    }

    fn visit_unary(
        &mut self,
        operator: &super::token::Token,
        right: &Expression,
        is_prefix: bool,
    ) -> Literal {
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

    fn visit_variable(&mut self, name: &String) -> Literal {
        if let Some(variable) = self.environment.access(name).clone() {
            return variable.clone();
        }

        Literal::None
    }
}

impl statement::Visitor<()> for Interpreter {
    fn visit_program(&mut self, statements: &Vec<Statement>) -> () {
        for statement in statements {
            statement.accept(self);
        }
    }

    fn visit_print(&mut self, expression: &Expression) -> () {
        println!("{}", expression.accept(self));
    }

    fn visit_expression(&mut self, expression: &Expression) -> () {
        expression.accept(self);
    }

    fn visit_variable_declaration(&mut self, name: &String, value: &Expression) -> () {
        let value = value.accept(self);
        self.environment.define(name.clone(), value);
    }
}
