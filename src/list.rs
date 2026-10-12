// ANCHOR: def_all
// ANCHOR: def
#[derive(Debug, Clone, Copy)]
struct List<const N: usize, T>
where
    T: Copy,
{
    element: [T; N],
}

impl<const N: usize, T> List<N, T>
where
    T: Copy,
{
    pub fn new(element: [T; N]) -> List<N, T> {
        List { element }
    }
}

impl<const N: usize, T> List<N, T>
where
    T: Copy,
{
    pub fn len() -> usize {
        N
    }
}
// ANCHOR_END: def

// ANCHOR: fmt_and_eq
impl<const N: usize, T> std::fmt::Display for List<N, T>
where
    T: std::fmt::Display + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, e) in self.element.iter().enumerate() {
            write!(f, "{}", e)?;
            if i + 1 < N {
                write!(f, ",")?;
            }
        }
        write!(f, ")")
    }
}

impl<const N: usize, const M: usize, T> PartialEq<List<M, T>> for List<N, T>
where
    T: PartialEq + Copy,
{
    fn eq(&self, other: &List<M, T>) -> bool {
        N == M && self.element.as_slice() == other.element.as_slice()
    }
}
// ANCHOR_END: fmt_and_eq
// ANCHOR_END: def_all

// ANCHOR: zero_f64
impl<const N: usize> List<N, f64> {
    pub fn zero() -> List<N, f64> {
        List { element: [0.0; N] }
    }
}
// ANCHOR_END: zero_f64

use crate::complex::Complex;
impl<const N: usize> List<N, Complex> {
    pub fn zero() -> List<N, Complex> {
        List {
            element: [Complex::zero(); N],
        }
    }
}

// ANCHOR: op
use std::ops::{Add, Div, Mul, Neg, Sub};
impl<const N: usize, T> Add for List<N, T>
where
    T: Add<Output = T> + Copy,
{
    type Output = List<N, T>;
    fn add(self, rhs: Self) -> Self::Output {
        List {
            element: std::array::from_fn(|i| self.element[i] + rhs.element[i]),
        }
    }
}
impl<const N: usize, T> Neg for List<N, T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = List<N, T>;
    fn neg(self) -> Self::Output {
        List {
            element: std::array::from_fn(|i| -self.element[i]),
        }
    }
}
impl<const N: usize, T> Sub for List<N, T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = List<N, T>;
    fn sub(self, rhs: Self) -> Self::Output {
        List {
            element: std::array::from_fn(|i| self.element[i] - rhs.element[i]),
        }
    }
}
impl<const N: usize, T, Scalar> Mul<Scalar> for List<N, T>
where
    T: Mul<Scalar, Output = T> + Copy,
    Scalar: Copy,
{
    type Output = List<N, T>;

    fn mul(self, rhs: Scalar) -> Self::Output {
        List {
            element: std::array::from_fn(|i| self.element[i] * rhs),
        }
    }
}
impl<const N: usize, T, Scalar> Div<Scalar> for List<N, T>
where
    T: Div<Scalar, Output = T> + Copy,
    Scalar: Copy,
{
    type Output = List<N, T>;

    fn div(self, rhs: Scalar) -> Self::Output {
        List {
            element: std::array::from_fn(|i| self.element[i] / rhs),
        }
    }
}
// ANCHOR_END: op

#[cfg(test)]
mod test {
    use crate::complex::Complex;

    use super::*;

    #[test]
    fn display() {
        assert_eq!(List::<0, f32>::new([]).to_string(), "()");
        assert_eq!(List::new([1]).to_string(), "(1)");
        assert_eq!(List::new([1, 2]).to_string(), "(1,2)");
        assert_eq!(List::new([1, 2, 3]).to_string(), "(1,2,3)");
        assert_eq!(
            List::new([Complex::new(1, 2), Complex::new(3, 4), Complex::new(5, 6)]).to_string(),
            "(1 + 2i,3 + 4i,5 + 6i)"
        );
    }

    #[test]
    fn eq() {
        assert_eq!(List::new([1, 2, 3]), List::new([1, 2, 3]));
        assert_ne!(List::new([1, 2]), List::new([1, 2, 3]));
        assert_ne!(List::new([1]), List::new([1, 1]));
    }

    #[test]
    fn add() {
        assert_eq!(
            List::new([1, 2, 3]) + List::new([4, 5, 6]),
            List::new([5, 7, 9])
        );
    }

    #[test]
    fn zero() {
        assert_eq!(List::<3, f64>::zero().to_string(), "(0,0,0)");
        assert_eq!(
            List::<3, Complex>::zero().to_string(),
            "(0 + 0i,0 + 0i,0 + 0i)"
        );

        assert_eq!(
            List::<3, f64>::new([1.0, 2.0, 3.0]) + List::<3, f64>::zero(),
            List::<3, f64>::new([1.0, 2.0, 3.0])
        );

        assert_eq!(
            List::new([Complex::new(1, 2), Complex::new(3, 4), Complex::new(5, 6)])
                + List::<3, Complex>::zero(),
            List::new([Complex::new(1, 2), Complex::new(3, 4), Complex::new(5, 6)])
        );
    }

    #[test]
    fn neg() {
        assert_eq!(-List::new([1, 2, 3]), List::new([-1, -2, -3]));
    }

    #[test]
    fn sub() {
        assert_eq!(
            List::new([5, 7, 9]) - List::new([4, 5, 6]),
            List::new([1, 2, 3])
        );
    }

    #[test]
    fn mul() {
        assert_eq!(List::new([1, 2, 3]) * 5, List::new([5, 10, 15]));
    }
}
