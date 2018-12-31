#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Defines when the checker should log its result.
pub enum LogCondition {
    /// The checker always logs its result.
    Always,
    /// The checker logs its result if and only if a test run has failed.
    OnFailure,
}

impl Default for LogCondition {
    fn default() -> Self {
        LogCondition::OnFailure
    }
}
