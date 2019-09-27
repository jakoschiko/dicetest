/// Defines when the checker should log the test result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogCondition {
    /// The checker always logs the test result.
    Always,
    /// The checker logs the test result if and only if a test run has failed.
    OnFailure,
}

impl Default for LogCondition {
    fn default() -> Self {
        LogCondition::OnFailure
    }
}
