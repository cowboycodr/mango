#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Plus,
    Minus,
    Slash,

    Equal,
    Bang,
    Star,
    StarStar,

    Print,
    Var,

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

            "var" => TokenType::Var,
            "print" => TokenType::Print,

            _ => TokenType::Identifier,
        }
    }
}
