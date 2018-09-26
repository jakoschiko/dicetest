use ::prop::{Labels, Status};

/// The result of a property evaluation.
#[derive(Debug, Clone)]
#[must_use]
pub struct Result {
    /// The truth value of the evaluated property.
    pub status: Status,
    /// Details about the property evaluation. Labels will not be created if `Params::create_labels`
    /// is set to `false`.
    pub labels: Labels,
}

impl Result {
    /// Creates a new `Result` with the given `Status` and an empty `Labels`.
    pub fn new(status: Status) -> Self {
        let labels = Labels::new();

        Result { status, labels }
    }

    /// Converts the `Result` to a human-readable summary.
    fn summary(&self) {
        let labels = self.labels.pretty_labels();
        format!("Status: {:?}\nLabels:\n{}", self.status, labels);
    }
}
