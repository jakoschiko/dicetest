use crate::prop::Eval;
use crate::brooder::{Status, EvalSummary};

/// Defines when the assertion should panic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panic {
    /// The assertion panics always regardless of the result.
    Always,
    /// The assertion panics when the property was neither proven nor did it pass.
    NotPassed,
    /// The assertion panics never regardless of the result.
    Never,
}

impl Panic {
    /// Decides if the assertion should panic based on the given `Eval`.
    pub fn should_panic_with_eval(self, eval: Eval) -> bool {
        match self {
            Panic::Always => true,
            Panic::NotPassed => {
                match eval {
                    Eval::True => false,
                    Eval::Passed => false,
                    _ => true,
                }
            }
            Panic::Never => false,
        }
    }

    /// Decides if the assertion should panic based on the given `Status`.
    pub fn should_panic_with_status(self, status: &Status) -> bool {
        match self {
            Panic::Always => true,
            Panic::NotPassed => {
                match status {
                    Status::Checked(ref eval_series) => {
                        match &eval_series.summary {
                            &EvalSummary::True => false,
                            &EvalSummary::Passed => false,
                            _ => true,
                        }
                    }
                    _ => true,
                }
            }
            Panic::Never => false,
        }
    }
}

impl Default for Panic {
    fn default() -> Self {
        Panic::NotPassed
    }
}
