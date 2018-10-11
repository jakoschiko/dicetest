use ::prop::Prints;
use ::checker::EvalParams;

/// The merged result of several property evaluations.
#[derive(Debug, Clone)]
pub enum EvalSummary {
    /// The property was evaluated to `prop::Status::True` at least once, but was never
    /// evaluated to `prop::Status::False`.
    True,
    /// The property was only evaluated to `prop::Status::Passed` so far.
    Passed,
    /// The property was evaluated to `prop::Status::False` at least once.
    False {
        /// The parameters that was used when the property was evaluted to `prop::Status::False`.
        counterexample: EvalParams,
        /// The prints that were collected when the property was evaluted to `prop::Status::False`.
        prints: Prints,
    },
}

impl EvalSummary {
    /// Merge operation for `EvalSummary`.
    ///
    /// This operation is similar to the logical conjunction.
    pub fn merge(self, other: EvalSummary) -> Self {
        use self::EvalSummary::{True, Passed, False};
        // A property should never or always evaluate to `prop::Status::True`.
        // Nevertheless we can't trust the property implementation and must handle all cases.
        match (self, other) {
            (True, True) => True,
            (Passed, True) => True,
            (True, Passed) => True,
            (Passed, Passed) => Passed,
            (True, f@False { .. }) => f,
            (Passed, f@False { .. }) => f,
            (f@False { .. }, True) => f,
            (f@False { .. }, Passed) => f,
            (
                False { counterexample: left_counterexample, prints: left_prints },
                False { counterexample: right_counterexample, prints: right_prints }
            ) => {
                // We prefer the counterexample that is probably smaller
                if left_counterexample.limit < right_counterexample.limit {
                    False { counterexample: left_counterexample, prints: left_prints }
                } else {
                    False { counterexample: right_counterexample, prints: right_prints }
                }
            }
        }
    }
}