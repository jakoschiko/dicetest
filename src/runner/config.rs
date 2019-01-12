use crate::die::Limit;
use crate::prand::Seed;

/// The configuration for running the test repeatedly.
#[derive(Debug, Clone)]
pub struct Config {
    /// The initial seed for the pseudorandom value generation. If `None` the runner uses a random
    /// seed.
    pub seed: Option<Seed>,
    /// The upper size limit of generated dynamic data structures used for the first test run.
    /// The following test runs use an interpolated limit between `start_limit` and `end_limit`.
    pub start_limit: Limit,
    /// The upper size limit of generated dynamic data structures used for the last test run.
    /// The previous test runs use an interpolated limit between `start_limit` and `end_limit`.
    pub end_limit: Limit,
    /// Defines how many times the test needs to run without failing.
    ///
    /// The runner aborts early if a counterexample has been found.
    pub passes: u64,
    /// Defines whether the counterexample will be rerun with enabled hints. The hints will be
    /// added to the report.
    ///
    /// This parameter does only work if the feature `hints` is present.
    pub hints_enabled: bool,
    /// Defines whether the stats will be enabled during the test runs. The stats will be added
    /// to the report.
    ///
    /// This parameter does only work if the feature `stats` is present.
    pub stats_enabled: bool,
}

impl Config {
    pub fn with_seed(self, seed: Option<Seed>) -> Self {
        Config { seed, ..self }
    }

    pub fn with_start_limit(self, start_limit: Limit) -> Self {
        Config {
            start_limit,
            ..self
        }
    }

    pub fn with_end_limit(self, end_limit: Limit) -> Self {
        Config { end_limit, ..self }
    }

    pub fn with_multiplied_limit(self, factor: f64) -> Self {
        Config {
            start_limit: multiply(self.start_limit.0, factor).into(),
            end_limit: multiply(self.end_limit.0, factor).into(),
            ..self
        }
    }

    pub fn with_passes(self, passes: u64) -> Self {
        Config { passes, ..self }
    }

    pub fn with_multiplied_passes(self, factor: f64) -> Self {
        Config {
            passes: multiply(self.passes, factor),
            ..self
        }
    }

    pub fn with_hints_enabled(self, hints_enabled: bool) -> Self {
        Config {
            hints_enabled,
            ..self
        }
    }

    pub fn with_stats_enabled(self, stats_enabled: bool) -> Self {
        Config {
            stats_enabled,
            ..self
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            seed: None,
            start_limit: 0.into(),
            end_limit: Limit::default(),
            passes: 1000,
            hints_enabled: true,
            stats_enabled: false,
        }
    }
}

fn multiply(value: u64, factor: f64) -> u64 {
    (value as f64 * factor) as u64
}
