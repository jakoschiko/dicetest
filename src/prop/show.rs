use std::fmt::Debug;

/// Trait for converting values of type `T` to human-readable texts.
pub trait Show<T> {
    /// Converts the value to a human-readable text.
    fn show(self, &T) -> String;
}

impl<T, F> Show<T> for F
where
    F: Fn(&T) -> String
{
    fn show(self, value: &T) -> String {
        self(value)
    }
}

/// Converts values to texts by using the `Debug` formatter.
pub struct DebugShow;

impl<T: Debug> Show<T> for DebugShow {
    fn show(self, value: &T) -> String {
        format!("{:?}", value)
    }
}
