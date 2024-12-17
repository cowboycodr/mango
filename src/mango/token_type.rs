#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,

    Plus,
    Minus,
    Star,
    Slash,

    Number,

    End,
}
