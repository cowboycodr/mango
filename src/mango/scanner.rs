use super::token::Token;
use super::token_type::TokenType;

use super::literal::Literal;

pub struct Source {
    input: String,
    pub position: usize,
}

impl Source {
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    pub fn next(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let char = self.input.chars().collect::<Vec<char>>()[self.position];
        self.position += 1;

        char
    }

    pub fn peek(&self, offset: usize) -> char {
        if self.position + offset >= self.input.len() {
            return '\0';
        }

        return self.input.chars().collect::<Vec<char>>()[self.position + offset];
    }

    pub fn slice(&mut self, start: usize, end: Option<usize>) -> &str {
        &self.input[start..end.unwrap_or(self.input.len())]
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

pub struct Scanner {
    source: Source,
    start: usize,
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Self {
            source: Source::new(input.into()),
            start: 0,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        while !self.source.is_at_end() {
            self.start = self.source.position;

            let token = self.scan_token();
            if let Some(token) = token {
                tokens.push(token);
            }
        }
        tokens.push(Token::new(TokenType::End, None));

        return tokens;
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        let char = self.source.next();

        match char {
            ' ' | '\t' | '\r' | '\n' => None, // Skip irrelevant, but valid characters, first.

            ';' => Some(Token::new(TokenType::Semicolon, None)),

            '(' => Some(Token::new(TokenType::LeftParen, None)),
            ')' => Some(Token::new(TokenType::RightParen, None)),
            '{' => Some(Token::new(TokenType::LeftBrace, None)),
            '}' => Some(Token::new(TokenType::RightBrace, None)),

            '+' => Some(Token::new(TokenType::Plus, None)),
            '-' => Some(Token::new(TokenType::Minus, None)),
            '/' => Some(Token::new(TokenType::Slash, None)),

            '=' => Some(Token::new(TokenType::Equal, None)),
            '!' => Some(Token::new(TokenType::Bang, None)),
            '*' => {
                if self.source.peek(0) == '*' {
                    self.source.next();
                    Some(Token::new(TokenType::StarStar, None))
                } else {
                    Some(Token::new(TokenType::Star, None))
                }
            }

            '"' => {
                while self.source.peek(0) != '"' && !self.source.is_at_end() {
                    self.source.next();
                }

                if self.source.is_at_end() && self.source.peek(0) != '"' {
                    panic!("Unterminated string..");
                }

                self.source.next();

                let value = self
                    .source
                    .slice(self.start + 1, Some(self.source.position - 1))
                    .to_string();
                Some(Token::new(TokenType::String, Some(Literal::String(value))))
            }
            c if c.is_ascii_digit() => {
                while self.source.peek(0).is_ascii_digit() && !self.source.is_at_end() {
                    self.source.next();
                }

                if self.source.peek(0) == '.' && self.source.peek(1).is_ascii_digit() {
                    self.source.next();

                    while self.source.peek(0).is_ascii_digit() {
                        self.source.next();
                    }
                }

                let value: f64 = self
                    .source
                    .slice(self.start, Some(self.source.position))
                    .parse()
                    .unwrap_or_default();

                Some(Token::new(TokenType::Number, Some(Literal::Number(value))))
            }
            c if c.is_alphabetic() => {
                while self.source.peek(0).is_alphanumeric() {
                    self.source.next();
                }

                let value: String = self
                    .source
                    .slice(self.start, Some(self.source.position))
                    .to_string();
                let kind = TokenType::from(value.clone());

                Some(Token::new(
                    TokenType::from(kind),
                    Some(Literal::String(value)),
                ))
            }
            c => panic!("'{c}' Unknown character!"),
        }
    }
}
