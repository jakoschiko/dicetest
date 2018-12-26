use crate::hints::Hints;
use crate::runner::{Run, Error};

// The result of a single test run.
pub struct Sample {
    /// The configuration used to create this result.
    pub run: Run,
    // The hints collected during the rest run.
    pub hints: Hints,
    // The error occurred during the rest run.
    pub error: Option<Error>,
}