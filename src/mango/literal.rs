use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub enum Literal {
    Number(f64),

    None,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::None => write!(f, "None"),
        }
    }
}

impl Add for Literal {
    type Output = Literal;

    fn add(self, other: Literal) -> Literal {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Number(a + b),
            _ => Literal::None,
        }
    }
}

impl Sub for Literal {
    type Output = Literal;

    fn sub(self, other: Literal) -> Literal {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Number(a - b),
            _ => Literal::None,
        }
    }
}

impl Mul for Literal {
    type Output = Literal;

    fn mul(self, other: Literal) -> Literal {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Number(a * b),
            _ => Literal::None,
        }
    }
}

impl Div for Literal {
    type Output = Literal;

    fn div(self, other: Literal) -> Literal {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) if b != 0.0 => Literal::Number(a / b),
            _ => Literal::None,
        }
    }
}

impl Neg for Literal {
    type Output = Literal;

    fn neg(self) -> Literal {
        match self {
            Literal::Number(a) => Literal::Number(-a),
            _ => Literal::None,
        }
    }
}
