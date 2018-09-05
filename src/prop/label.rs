use std::string::ToString;

/// Labels provides internal details of an evaluated property. E.g. the label may contain a local
/// variable or a clue which branch was taken. The most important use case is the debugging of
/// falsified properties.
///
/// As far as possible labels should be created lazily and should not slow down the test execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    /// A human-readable text.
    pub text: String
}

/// Trait for converting data into labels. Useful for creating labels lazily.
pub trait IntoLabel {
    fn into_label(self) -> Label;
}

impl IntoLabel for Label {
    fn into_label(self) -> Label {
        self
    }
}

impl<'a> IntoLabel for &'a str {
    fn into_label(self) -> Label {
        let text = self.to_string();
        Label { text }
    }
}

impl IntoLabel for String {
    fn into_label(self) -> Label {
        let text = self;
        Label { text }
    }
}

impl<F> IntoLabel for F
where
    F: FnOnce() -> String,
{
    fn into_label(self) -> Label {
        let text = self();
        Label { text }
    }
}

/// A never type for labels.
pub enum NoLabel {}

impl IntoLabel for NoLabel {
    fn into_label(self) -> Label {
        match self {}
    }
}

/// A collection of labels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Labels {
    pub labels: Vec<Label>,
}

impl Labels {
    /// Creates a new empty instance.
    pub fn new() -> Self {
        let labels = Vec::new();
        Labels { labels }
    }

    /// Evaluates and pushes the label to the back of self.
    pub fn push<L>(&mut self, label: L)
    where
        L: IntoLabel,
    {
        let label = label.into_label();
        self.labels.push(label);
    }

    /// Appends the given labels to self.
    pub fn append(&mut self, labels: &mut Labels) {
        self.labels.append(&mut labels.labels);
    }

    /// Indents the given labels and appends them to self.
    pub fn append_indented(&mut self, labels: &mut Labels) {
        for label in labels.labels.iter_mut() {
            *label = format!("\t{}", label.text).into_label()
        }

        self.labels.append(&mut labels.labels);
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
}
