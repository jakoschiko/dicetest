use std::time::Duration;

use ::gen;

/// The parameters for the checker.
#[derive(Debug, Clone)]
pub struct Params {
    /// The initial seed for the random value generation. If `None` the checker uses a random seed.
    pub seed: Option<u64>,
    // The maxmimum size of generated dynamic data structures used for the first property
    // evaluation. The next property evaluations use an interpolated maximum size between
    // `start_size` and `end_size`.
    //
    // The maximum size will be based to the generators, see `gen::Parmas::size`.
    pub start_size: u64,
    // The maxmimum size of generated dynamic data structures used for the last property
    // evaluation. The previous property evaluations use an interpolated maximum size between
    // `start_size` and `end_size`.
    //
    // The maximum size will be based to the generators, see `gen::Parmas::size`.
    pub end_size: u64,
    /// The upper limit for the number of property evaluations.
    ///
    /// If the property evalutes to `prop::Status::Passed` and `min_passed` is not reached,
    /// the checker evaluates the property again. In all other cases, the checker is finished.
    pub min_passed: u64,
    /// The number of worker threads used for evaluating the property.
    ///
    /// If set to `0`, no worker trhead is used and the property is evaluated in the caller thread.
    /// In this case, the checker cannot handle timeouts or panicked threads.
    pub worker_count: u64,
    /// A timeout for the worker threads. If the timeout is reached, the checker aborts the
    /// evaluation even though there are workers still running. `None` means unlimited time.
    ///
    /// The timeout will be ignored if `worker_count` is set to `0`.
    pub timeout: Option<Duration>,
}

impl Params {
    /// Sets the field `seed`.
    pub fn seed(self, seed: Option<u64>) -> Self {
        Params { seed, ..self }
    }

    /// Sets the field `start_size`.
    pub fn start_size(self, start_size: u64) -> Self {
        Params { start_size, ..self }
    }

    /// Sets the field `end_size`.
    pub fn end_size(self, end_size: u64) -> Self {
        Params { end_size, ..self }
    }

    /// Sets the field `min_passed`.
    pub fn min_passed(self, min_passed: u64) -> Self {
        Params { min_passed, ..self }
    }

    /// Sets the field `worker_count`.
    pub fn worker_count(self, worker_count: u64) -> Self {
        Params { worker_count, ..self }
    }

    /// Sets the field `timeout`.
    pub fn timeout(self, timeout: Option<Duration>) -> Self {
        Params { timeout, ..self }
    }
}

impl Default for Params {
    fn default() -> Self {
        Params {
            seed: None,
            start_size: 0,
            end_size: gen::Params::default().size,
            min_passed: 1000,
            worker_count: 1,
            timeout: None,
        }
    }
}
