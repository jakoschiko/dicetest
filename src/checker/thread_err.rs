use std::any::Any;
use std::rc::Rc;

/// Wraps an error received from a panicked thread.
#[derive(Debug, Clone)]
pub struct ThreadErr {
    /// The thread panicked with this error.
    pub error: Rc<Any + Send + 'static>
}

impl ThreadErr {
    /// Wraps the given error.
    pub fn new(error_box: Box<Any + Send + 'static>) -> Self {
        let error = error_box.into();
        ThreadErr { error }
    }

    /// Tries to interpret the error as a string.
    pub fn error_string(&self) -> Option<&str> {
        let error = &self.error;

        if let Some(string) = error.downcast_ref::<String>() {
            Some(string)
        } else if let Some(str) = error.downcast_ref::<&str>() {
            Some(str)
        } else {
            None
        }
    }
}
