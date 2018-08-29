use std::string::ToString;

/// Labels provides internal details of an evaluated property. E.g. the label may contain a local
/// variable or a clue which branch was taken. The most important use case is the debugging of
/// falsified properties.
///
/// As far as possible labels should be created lazily and should not slow down the test execution.
#[derive(Debug, PartialEq)]
pub struct Label {
	/// A human-readable text.
	pub text: String
}

/// Trait for converting data into labels. Useful for creating labels lazily.
pub trait IntoLabel {
	fn into_label(self) -> Label;
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
