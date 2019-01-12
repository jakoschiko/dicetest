use std::any::Any;

/// Contains an error that occurred during a test run.
#[derive(Debug)]
pub struct Error(pub Box<dyn Any + Send + 'static>);
