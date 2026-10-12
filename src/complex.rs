// ANCHOR: all

// ANCHOR: def
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

// ANCHOR_END: all
