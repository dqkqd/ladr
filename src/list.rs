// ANCHOR: def_all
// ANCHOR: def
#[derive(Debug)]
struct List<const N: usize, T> {
    element: [T; N],
}

impl<const N: usize, T> List<N, T> {
    pub fn new(element: [T; N]) -> List<N, T> {
        List { element }
    }
}

impl<const N: usize, T> List<N, T> {
    pub fn len() -> usize {
        N
    }
}
// ANCHOR_END: def

// ANCHOR: fmt_and_eq
impl<const N: usize, T> std::fmt::Display for List<N, T>
where
    T: std::fmt::Display,
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
    T: PartialEq,
{
    fn eq(&self, other: &List<M, T>) -> bool {
        N == M && self.element.as_slice() == other.element.as_slice()
    }
}
// ANCHOR_END: fmt_and_eq
// ANCHOR_END: def_all

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
            List::new([
                Complex::new(1.0, 2.0),
                Complex::new(3.0, 4.0),
                Complex::new(5.0, 6.0)
            ])
            .to_string(),
            "(1 + 2i,3 + 4i,5 + 6i)"
        );
    }

    #[test]
    fn eq() {
        assert_eq!(List::new([1, 2, 3]), List::new([1, 2, 3]));
        assert_ne!(List::new([1, 2]), List::new([1, 2, 3]));
        assert_ne!(List::new([1]), List::new([1, 1]));
    }
}
