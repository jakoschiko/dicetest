use std::string::ToString;

/// Trait for lazy string creation.
pub trait LazyString {
    fn create_string(self) -> String;
}

impl LazyString for &'static str {
    fn create_string(self) -> String {
        self.to_string()
    }
}

impl<S, F> LazyString for F
where
    S: Into<String>,
    F: FnOnce() -> S,
{
    fn create_string(self) -> String {
        self().into()
    }
}

/// A never type for `LazyString`.
pub enum NoString {}

impl LazyString for NoString {
    fn create_string(self) -> String {
        match self {}
    }
}
