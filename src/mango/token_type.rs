#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,

    Plus,
    Minus,
    Slash,

    Bang,
    Star,
    StarStar,

    True,
    False,

    Number,
    Identifier,
    String,

    End,
}

impl From<String> for TokenType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Identifier,
        }
    }
}
