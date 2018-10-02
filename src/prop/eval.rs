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