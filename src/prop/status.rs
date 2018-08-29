/// Extended truth values for property evaluation.
pub enum Status {
    /// The property was proven to be true.
    True,
    /// The property withstood a test without being falsified. Useful for describing the result of
    /// a partially evaluated universal quantifier.
    Passed,
    /// The property was proven to be false.
    False,
}
