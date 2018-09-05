use ::prop::{Labels, Status};

/// The result of a property evaluation.
#[derive(Debug, Clone)]
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

    /// Prints the `Result` in a pretty format to stdout.
    pub fn print(&self) {
        let labels = self.labels.pretty_labels();
        println!("Status: {:?}\nLabels:\n{}", self.status, labels);
    }
}
