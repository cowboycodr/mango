use crate::mango::literal::Literal;

use super::expression::Expression;

use super::token::Token;
use super::token_type::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Expression {
        self.expression()
    }

    fn expression(&mut self) -> Expression {
        self.term()
    }

    fn term(&mut self) -> Expression {
        let mut expression = self.factor();

        if self.expect(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.term();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }

        expression
    }

    fn factor(&mut self) -> Expression {
        let mut expression = self.exponent();

        if self.expect(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.factor();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }

        expression
    }

    fn exponent(&mut self) -> Expression {
        let mut expression = self.unary();

        if self.expect(&[TokenType::StarStar]) {
            let operator = self.previous();
            let right = self.exponent();

            expression = Expression::Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            };
        }

        expression
    }

    fn unary(&mut self) -> Expression {
        if self.expect(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();

            return Expression::Unary {
                operator,
                right: Box::new(right),
                is_prefix: true,
            };
        }

        let mut expression = self.primary();

        if self.expect(&[TokenType::Bang]) {
            let operator = self.previous();

            expression = Expression::Unary {
                operator: operator,
                right: Box::new(expression),
                is_prefix: false,
            };
        }

        expression
    }

    fn primary(&mut self) -> Expression {
        if self.expect(&[TokenType::True]) {
            return Expression::Literal(Literal::Boolean(true));
        }
        if self.expect(&[TokenType::False]) {
            return Expression::Literal(Literal::Boolean(false));
        }
        if self.expect(&[TokenType::Number]) {
            return Expression::Literal(self.previous().literal);
        }
        if self.expect(&[TokenType::LeftParen]) {
            let expression = Expression::Grouping {
                expression: Box::new(self.expression()),
            };
            self.consume(
                TokenType::RightParen,
                "')' Expected closing parenthesis".to_string(),
            );
            return expression;
        }

        panic!("Uhh ohh {:?}", self.peek(0));
    }

    fn previous(&self) -> Token {
        self.tokens[self.position - 1].clone()
    }

    fn expect(&mut self, types: &[TokenType]) -> bool {
        for kind in types {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, kind: &TokenType) -> bool {
        self.peek(0).kind == *kind
    }

    fn peek(&self, offset: usize) -> Token {
        if self.position + offset > self.tokens.len() {
            return self.tokens.last().unwrap().clone();
        }
        self.tokens[self.position + offset].clone()
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            return self.peek(0);
        }

        let token = self.tokens[self.position].clone();
        self.position += 1;

        return token;
    }

    fn consume(&mut self, kind: TokenType, message: String) -> Token {
        if self.check(&kind) {
            return self.advance();
        }

        panic!("{message}");
    }

    fn is_at_end(&self) -> bool {
        self.peek(0).kind == TokenType::End
    }
}
