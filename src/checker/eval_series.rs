use ::prop::{Prints, Eval};
use ::checker::{EvalParams, EvalSummary};

/// Contains the results of a series of property evaluations.
#[derive(Debug, Clone)]
#[must_use]
pub struct EvalSeries {
    /// The merged result of all property evaluations.
    pub summary: EvalSummary,
    /// The number of property evaluations with the result `Eval::Passed`.
    pub passed_tests: u64,
}

impl EvalSeries {
    /// Creates a new instance without any property evaluations.
    pub fn new() -> Self {
        EvalSeries {
            // `EvalSummary::Passed` is the most neutral element of `EvalSummary::merge`
            summary: EvalSummary::Passed,
            passed_tests: 0,
        }
    }

    /// Creates an instance for a single property evaluation.
    pub fn from_eval(
        eval: Eval,
        prints: Prints,
        eval_params: impl FnOnce() -> EvalParams,
    ) -> Self {
        match eval {
            Eval::True => {
                EvalSeries {
                    summary: EvalSummary::True,
                    passed_tests: 0,
                }
            }
            Eval::Passed => {
                EvalSeries {
                    summary: EvalSummary::Passed,
                    passed_tests: 1,
                }
            }
            Eval::False => {
                EvalSeries {
                    summary: EvalSummary::False {
                        counterexample: eval_params(),
                        prints,
                    },
                    passed_tests: 0,
                }
            }
        }
    }

    /// Merge opertation for `EvalSeries`.
    pub fn merge(self, other: Self) -> Self {
        let summary = self.summary.merge(other.summary);
        let passed_tests = self.passed_tests + other.passed_tests;
        EvalSeries { summary, passed_tests }
    }
}
