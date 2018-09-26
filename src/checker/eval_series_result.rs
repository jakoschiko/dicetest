use ::prop;
use ::checker::{EvalParams, EvalSeriesStatus};

/// The merged result of several property evaluations.
#[derive(Debug, Clone)]
#[must_use]
pub struct EvalSeriesResult {
    /// The merged status of all property evaluations.
    pub status: EvalSeriesStatus,
    /// The number of property evaluations with the status `prop::Status::Passed`.
    pub passed_tests: u64,
}

impl EvalSeriesResult {
    /// Creates a new instance without any property evaluations.
    pub fn new() -> Self {
        EvalSeriesResult {
            // `EvalSeriesStatus::Passed` is the most neutral element of `EvalSeriesStatus::merge`
            status: EvalSeriesStatus::Passed,
            passed_tests: 0,
        }
    }

    /// Creates an instance for a single property evaluation.
    pub fn from_prop_result(
        prop_result: prop::Result,
        eval_params: impl FnOnce() -> EvalParams,
    ) -> Self {
        match prop_result.status {
            prop::Status::True => {
                EvalSeriesResult {
                    status: EvalSeriesStatus::True,
                    passed_tests: 0,
                }
            }
            prop::Status::Passed => {
                EvalSeriesResult {
                    status: EvalSeriesStatus::Passed,
                    passed_tests: 1,
                }
            }
            prop::Status::False => {
                EvalSeriesResult {
                    status: EvalSeriesStatus::False {
                        counterexample: eval_params(),
                        labels: prop_result.labels,
                    },
                    passed_tests: 0,
                }
            }
        }
    }

    /// Merges both instances into one.
    pub fn merge(self, other: Self) -> Self {
        let status = self.status.merge(other.status);
        let passed_tests = self.passed_tests + other.passed_tests;
        EvalSeriesResult { status, passed_tests }
    }
}
