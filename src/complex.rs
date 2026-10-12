// ANCHOR: all

// ANCHOR: def
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    a: f64,
    b: f64,
}

impl Complex {
    pub fn new(a: impl Into<f64>, b: impl Into<f64>) -> Complex {
        Complex {
            a: a.into(),
            b: b.into(),
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.a, self.b)
    }
}
// ANCHOR_END: def

// ANCHOR: zero
impl Complex {
    pub fn zero() -> Complex {
        Complex { a: 0.0, b: 0.0 }
    }
}
// ANCHOR_END: zero

// ANCHOR: one
impl Complex {
    pub fn one() -> Complex {
        Complex { a: 1.0, b: 0.0 }
    }
}
// ANCHOR_END: one

// ANCHOR: inv
impl Complex {
    pub fn inv(&self) -> Complex {
        let denom = self.a * self.a + self.b * self.b;
        Complex {
            a: self.a / denom,
            b: -self.b / denom,
        }
    }
}
// ANCHOR_END: inv

// ANCHOR: pow
impl Complex {
    pub fn pow(&self, n: u32) -> Complex {
        if n == 0 {
            return Complex::one();
        }
        let c = self.pow(n / 2);
        match n.is_multiple_of(2) {
            true => c * c,
            false => c * c * *self,
        }
    }
}
// ANCHOR_END: pow

// ANCHOR: op
use std::ops::{Add, Div, Mul, Neg, Sub};

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
impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Complex {
            a: -self.a,
            b: -self.b,
        }
    }
}
impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Self) -> Self::Output {
        self.mul(rhs.inv())
    }
}
// ANCHOR_END: op

// ANCHOR_END: all

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        let c = Complex::new(3, 4);
        assert_eq!(c.to_string(), "3 + 4i");
    }

    #[test]
    fn add() {
        let lhs = Complex::new(2, 3);
        let rhs = Complex::new(4, 5);
        assert_eq!((lhs + rhs).to_string(), "6 + 8i");
    }

    #[test]
    fn mul() {
        let lhs = Complex::new(2, 3);
        let rhs = Complex::new(4, 5);
        assert_eq!((lhs * rhs).to_string(), "-7 + 22i");
    }

    #[test]
    fn zero() {
        let c = Complex::new(2, 3);
        assert_eq!(c + Complex::zero(), c);
    }

    #[test]
    fn one() {
        let c = Complex::new(2, 3);
        assert_eq!(c * Complex::one(), c);
    }

    #[test]
    fn neg() {
        let c = Complex::new(2, 3);
        let neg_c = -c;
        assert_eq!(neg_c.to_string(), "-2 + -3i");
        assert_eq!(c + neg_c, Complex::zero());
    }

    #[test]
    fn inv() {
        let c = Complex::new(1, 2);
        assert_eq!(c.inv().to_string(), "0.2 + -0.4i");
        assert_eq!(c * c.inv(), Complex::one());
    }

    #[test]
    fn sub() {
        let lhs = Complex::new(3, 6);
        let rhs = Complex::new(1, 2);
        assert_eq!((lhs - rhs).to_string(), "2 + 4i");
    }

    #[test]
    fn div() {
        let lhs = Complex::new(2, 8);
        let rhs = Complex::new(1, 2);
        assert_eq!(Complex::one() / rhs, rhs.inv());
        assert_eq!((lhs / rhs) * rhs, lhs);
    }

    #[test]
    fn pow() {
        let c = Complex::new(1, 2);
        assert_eq!(c.pow(0), Complex::one());
        assert_eq!(c.pow(1), c);
        assert_eq!(c.pow(2), c * c);
        assert_eq!(c.pow(3), c * c * c);
        assert_eq!(c.pow(4), c * c * c * c);
        assert_eq!(c.pow(5), c * c * c * c * c);
        assert_eq!(c.pow(6), c * c * c * c * c * c);
        assert_eq!(c.pow(7), c * c * c * c * c * c * c);
        assert_eq!(c.pow(8), c * c * c * c * c * c * c * c);
    }
}
