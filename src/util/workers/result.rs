use std::any::Any;

/// The result of a single function.
pub struct SingleResult<R> {
    /// The index of the function.
    pub index: u64,
    /// The result of the function or the error of the panicked thread.
    pub result: Result<R, Box<Any + Send + 'static>>
}

/// The results of multiple functions.
pub struct JoinedResult<R> {
    /// `false` iff all functions are terminated before the timeout.
    pub timeout: bool,
    /// The results of the functions that are terminated before the timeout.
    pub results: Vec<SingleResult<R>>,
}

impl<R> JoinedResult<R> {
    /// Creates an empty instance.
    pub fn new() -> Self {
        JoinedResult {
            timeout: false,
            results: Vec::new(),
        }
    }

    /// Returns a vector of the successful results if there are no errors. Otherwise returns the
    /// first error.
    pub fn oks_or_first_err(self) -> Result<Vec<R>, Box<Any + Send + 'static>> {
        self.results
            .into_iter()
            .map(|single| single.result)
            .collect::<Result<Vec<_>, _>>()
    }
}
