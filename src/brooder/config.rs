use std::time::Duration;

use crate::gen::Limit;

/// The configuration for the brooder.
#[derive(Debug, Clone)]
pub struct Config {
    /// The initial seed for the random value generation. If `None` the brooder uses a random seed.
    pub seed: Option<u64>,
    // The upper size limit of generated dynamic data structures used for the first property
    // evaluation. The next property evaluations use an interpolated limit between `start_limit`
    // and `end_limit`.
    //
    // The limit will be passed to the generators, see `gen::Limit`.
    pub start_limit: u64,
    // The upper size limit of generated dynamic data structures used for the last property
    // evaluation. The previous property evaluations use an interpolated limit between `start_limit`
    // and `end_limit`.
    //
    // The limit will be passed to the generators, see `gen::Limit`.
    pub end_limit: u64,
    /// The upper limit for the number of property evaluations.
    ///
    /// If the property evalutes to `prop::Status::Passed` and `min_passed` is not reached,
    /// the brooder evaluates the property again. In all other cases, the brooder is finished.
    pub min_passed: u64,
    /// The number of worker threads used for evaluating the property.
    ///
    /// If set to `0`, no worker thread is used and the property is evaluated in the caller thread.
    /// In this case, the brooder cannot handle timeouts.
    pub worker_count: u64,
    /// A timeout for the worker threads. If the timeout is reached, the brooder aborts the
    /// evaluation even though there are workers still running. `None` means unlimited time.
    ///
    /// The timeout will be ignored if `worker_count` is set to `0`.
    pub timeout: Option<Duration>,
    /// Defines whether the counter will be enabled during the property evaluations.
    pub counter_enabled: bool,
}

impl Config {
    /// Sets the field `seed`.
    pub fn with_seed(self, seed: Option<u64>) -> Self {
        Config { seed, ..self }
    }

    /// Sets the field `start_limit`.
    pub fn with_start_limit(self, start_limit: u64) -> Self {
        Config { start_limit, ..self }
    }

    /// Sets the field `end_limit`.
    pub fn with_end_limit(self, end_limit: u64) -> Self {
        Config { end_limit, ..self }
    }

    /// Sets the field `min_passed`.
    pub fn with_min_passed(self, min_passed: u64) -> Self {
        Config { min_passed, ..self }
    }

    /// Sets the field `worker_count`.
    pub fn with_worker_count(self, worker_count: u64) -> Self {
        Config { worker_count, ..self }
    }

    /// Sets the field `timeout`.
    pub fn with_timeout(self, timeout: Option<Duration>) -> Self {
        Config { timeout, ..self }
    }

    /// Sets the field `counter_enabled`.
    pub fn with_counter_enabled(self, counter_enabled: bool) -> Self {
        Config { counter_enabled, ..self }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            seed: None,
            start_limit: 0,
            end_limit: Limit::default().0,
            min_passed: 1000,
            worker_count: 0,
            timeout: None,
            counter_enabled: false,
        }
    }
}
