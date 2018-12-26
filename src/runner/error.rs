use std::any::Any;

/// Contains an error that occurred during a test run.
pub struct Error(pub Box<dyn Any + Send + 'static>);
