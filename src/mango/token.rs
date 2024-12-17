use super::literal::Literal;
use super::token_type::TokenType;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub kind: TokenType,
    pub literal: Literal,
}

impl Token {
    pub fn new(kind: TokenType, literal: Option<Literal>) -> Self {
        Self {
            kind,
            literal: literal.unwrap_or(Literal::None),
        }
    }
}
