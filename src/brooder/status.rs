use crate::brooder::{EvalSeries, ThreadErr};

// Differentiate whether the brooder finished or aborted.
#[derive(Debug, Clone)]
pub enum Status {
    /// The brooder finished regularly.
    Checked(EvalSeries),
    /// The brooder aborted because a worker thread panicked during property evaluation.
    Panic(ThreadErr),
    /// The brooder aborted because the timeout was reached.
    Timeout
}
