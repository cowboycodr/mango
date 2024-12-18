#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,

    Plus,
    Minus,
    Slash,

    Star,
    StarStar,

    Number,

    End,
}
