use ::checker::{EvalSeriesResult, ThreadErr};

// Represents either a finished or an aborted property check.
#[derive(Debug, Clone)]
pub enum Status {
    /// The property check finished regularly.
    Checked(EvalSeriesResult),
    /// The property check was aborted because a worker thread panicked during property evaluation.
    Panic(ThreadErr),
    /// The property check was aborted because the timeout was reached.
    Timeout
}
