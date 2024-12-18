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

    Number,

    End,
}
