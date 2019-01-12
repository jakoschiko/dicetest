use crate::hints::Hints;
use crate::runner::{Error, Run};

/// The result of a single test run.
#[derive(Debug)]
pub struct Sample {
    /// The configuration used to create this result.
    pub run: Run,
    /// The hints collected during the test run.
    pub hints: Hints,
    /// The error occurred during the test run.
    pub error: Option<Error>,
}
