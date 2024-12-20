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
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Star,
    StarStar,

    Var,
    Print,
    While,

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
            "while" => TokenType::While,

            _ => TokenType::Identifier,
        }
    }
}
