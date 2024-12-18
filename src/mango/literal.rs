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
    type Output = Self;

    fn add(self, other: Literal) -> Self {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Number(a + b),
            _ => Literal::None,
        }
    }
}

impl Sub for Literal {
    type Output = Self;

    fn sub(self, other: Literal) -> Self {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Number(a - b),
            _ => Literal::None,
        }
    }
}

impl Mul for Literal {
    type Output = Self;

    fn mul(self, other: Literal) -> Self {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Number(a * b),
            _ => Literal::None,
        }
    }
}

impl Div for Literal {
    type Output = Self;

    fn div(self, other: Literal) -> Self {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) if b != 0.0 => Literal::Number(a / b),
            _ => Literal::None,
        }
    }
}

impl Neg for Literal {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Literal::Number(a) => Literal::Number(-a),
            _ => Literal::None,
        }
    }
}

pub trait Pow {
    type Output;
    fn pow(self, exponent: Literal) -> Self::Output;
}

impl Pow for Literal {
    type Output = Self;

    fn pow(self, exponent: Literal) -> Self {
        match (self, exponent) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Number(a.powf(b)),
            _ => Literal::Number(0.0),
        }
    }
}