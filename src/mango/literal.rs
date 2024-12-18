use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Clone, Copy, Debug)]
pub enum Literal {
    Number(f64),
    Boolean(bool),

    None,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
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

impl Not for Literal {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Literal::Boolean(a) => Literal::Boolean(!a),
            _ => Literal::Boolean(false),
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
            _ => Literal::None,
        }
    }
}

pub trait Fac {
    type Output;
    fn fac(self) -> Self::Output;
}

impl Fac for Literal {
    type Output = Self;

    fn fac(self) -> Self::Output {
        match self {
            Literal::Number(a) => {
                let n = a as i64;
                if n < 1 {
                    return Literal::Number(0.0);
                }

                let mut result: i64 = 1;
                for i in 1..=a as i64 {
                    result *= i;
                }
                Literal::Number(result as f64)
            }
            _ => Literal::None,
        }
    }
}
