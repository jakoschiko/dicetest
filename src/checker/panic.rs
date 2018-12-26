#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Defines when the checker should panic.
pub enum Panic {
    /// The checker panics always.
    Always,
    /// The checker panics once a test run has failed.
    OnFailure,
}

impl Default for Panic {
    fn default() -> Self {
        Panic::OnFailure
    }
}
