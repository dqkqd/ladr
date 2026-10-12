// ANCHOR: all

// ANCHOR: def
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    a: f64,
    b: f64,
}
// ANCHOR_END: def

// ANCHOR: format
impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.a, self.b)
    }
}
// ANCHOR_END: format

// ANCHOR: add_and_mul
use std::ops::{Add, Mul};
impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}
impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            a: self.a * rhs.a - self.b * rhs.b,
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}
// ANCHOR_END: add_and_mul

// ANCHOR_END: all

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        let c = Complex { a: 3.0, b: 4.0 };
        assert_eq!(c.to_string(), "3 + 4i");
    }

    #[test]
    fn add() {
        let lhs = Complex { a: 2.0, b: 3.0 };
        let rhs = Complex { a: 4.0, b: 5.0 };
        assert_eq!((lhs + rhs).to_string(), "6 + 8i");
    }

    #[test]
    fn mul() {
        let lhs = Complex { a: 2.0, b: 3.0 };
        let rhs = Complex { a: 4.0, b: 5.0 };
        assert_eq!((lhs * rhs).to_string(), "-7 + 22i");
    }
}
