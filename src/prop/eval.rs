/// The result of a property evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum Eval {
    /// The property was proven to be true.
    ///
    /// A property should either never or always evaluate to `True`.
    True,
    /// The property withstood a test without being falsified.
    ///
    /// Useful for describing the result of a partially evaluated universal quantifier.
    Passed,
    /// The property was proven to be false.
    False,
}

impl Eval {
    // The logical conjunction of two `Eval`s.
    pub fn and(self, rhs: Self) -> Self {
        use self::Eval::{True, Passed, False};

        match (self, rhs) {
            (True, True) => True,
            (Passed, True) => True,
            (True, Passed) => True,
            (Passed, Passed) => Passed,
            (True, False) => False,
            (Passed, False) => False,
            (False, True) => False,
            (False, Passed) => False,
            (False, False) => False,
        }
    }
}
