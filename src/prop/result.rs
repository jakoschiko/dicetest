use ::prop::{Label, IntoLabel, Status};

/// The result of a property evaluation.
#[derive(Debug, Clone)]
pub struct Result {
    /// The truth value of the evaluated property.
    pub status: Status,
    /// Details of the evaluated property. Labels will only be created if `Params::create_labels`
    /// is set to `true`.
    pub labels: Vec<Label>,
}

impl Result {
    /// Creates a new `Result` with the given `Status` an no `Label`s.
    pub fn new(status: Status) -> Self {
        let labels = Vec::new();

        Result {
            status,
            labels,
        }
    }

    /// Appends the given label to the `Result`. The label will be created immediately.
    pub fn append_label<L>(&mut self, label: L)
    where
        L: IntoLabel,
    {
        let label = label.into_label();
        self.labels.push(label);
    }

    /// Appends the given labels to the `Result`.
    pub fn append_labels(&mut self, mut labels: Vec<Label>) {
        self.labels.append(&mut labels);
    }

    /// Indents the given labels and appends them to the `Result`.
    pub fn append_labels_indented(&mut self, mut labels: Vec<Label>) {
        for label in labels.iter_mut() {
            *label = format!("\t{}", label.text).into_label()
        }

        self.labels.append(&mut labels);
    }

    /// Returns a `String` that contains all labels in a pretty format.
    pub fn pretty_labels(&self) -> String {
        let mut acc = String::new();
        let mut iter = self.labels.iter();

        let add_label = |acc: &mut String, label: &Label| {
            acc.push_str(&"|");
            acc.push_str(&label.text);
        };

        if let Some(label) = iter.next() {
            add_label(&mut acc, label);
        }

        for label in iter {
            acc.push('\n');
            add_label(&mut acc, label);
        }

        acc
    }

    /// Prints the `Result` in a pretty format to stdout.
    pub fn print(&self) {
        let labels = self.pretty_labels();
        println!("Status: {:?}\nLabels:\n{}", self.status, labels);
    }
}
