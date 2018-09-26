use ::prop::Labels;
use ::checker::EvalParams;

/// The merged status of several property evaluations.
#[derive(Debug, Clone)]
pub enum EvalSeriesStatus {
    /// The property was evaluated to `prop::Status::True` at least once, but was never
    /// evaluated to `prop::Status::False`.
    True,
    /// The property was only evaluated to `prop::Status::Passed` so far.
    Passed,
    /// The property was evaluated to `prop::Status::False` at least once.
    False {
        /// The paramters that was used when the property was evaluted to `prop::Status::False`.
        counterexample: EvalParams,
        /// The labels that were generated when the property was evaluted to `prop::Status::False`.
        labels: Labels,
    },
}

impl EvalSeriesStatus {
    /// Merges both states into one.
    ///
    /// The merge operation is similar to the logical conjunction.
    pub fn merge(self, other: Self) -> Self {
        use self::EvalSeriesStatus::{True, Passed, False};
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
                False { counterexample: left_counterexample, labels: left_labels },
                False { counterexample: right_counterexample, labels: right_labels }
            ) => {
                // We prefer the counterexample that is probably smaller
                let size = |eval_params: &EvalParams| eval_params.gen_params.size;
                if size(&left_counterexample) < size(&right_counterexample) {
                    False { counterexample: left_counterexample, labels: left_labels }
                } else {
                    False { counterexample: right_counterexample, labels: right_labels }
                }
            }
        }
    }
}
