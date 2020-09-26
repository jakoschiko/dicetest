//! Provides a runner function that runs a test once.
//!
//! This runner function can be used for debugging a counterexample
//! that was found with `runner::repeatedly::run`.

use std::panic::{catch_unwind, UnwindSafe};

use crate::hints::Hints;
use crate::runner;
use crate::runner::Error;
use crate::stats::Stats;
use crate::{Fate, Limit, Prng};

/// The configuration for a single test run.
#[derive(Debug, Clone)]
pub struct Config {
    /// The upper limit for the size of generated dynamic data structures used for the test run.
    pub limit: Limit,
    /// Defines whether the test will be run with enabled hints. The hints will be
    /// added to the report.
    ///
    /// This parameter does only work if the feature `hints` is present.
    pub hints_enabled: bool,
    /// Defines whether the stats will be enabled during the test run. The stats will be added
    /// to the report.
    ///
    /// This parameter does only work if the feature `stats` is present.
    pub stats_enabled: bool,
}

/// The result of a single test run.
#[derive(Debug)]
pub struct Report {
    /// The hints collected during the test run. It's defined if and only if hints are enabled.
    pub hints: Option<Hints>,
    /// The stats collected during the test run. It's defined if and only if stats are enabled.
    pub stats: Option<Stats>,
    /// The error occurred during the test run. It's defined if and only the test has panicked.
    pub error: Option<Error>,
}

/// Runs the test once with the given configuration.
///
/// If the test panics the error will be caught and added to the report.
pub fn run<T>(mut prng: Prng, config: &Config, test: T) -> Report
where
    T: FnOnce(Fate) + UnwindSafe,
{
    let ((test_result, hints), stats) = {
        let limit = config.limit;
        runner::util::collect_stats(config.stats_enabled, || {
            runner::util::collect_hints(config.hints_enabled, || {
                catch_unwind(move || {
                    let fate = Fate::new(&mut prng, limit);
                    test(fate)
                })
            })
        })
    };

    let error = test_result.err().map(Error);

    Report {
        hints,
        stats,
        error,
    }
}

#[cfg(test)]
mod tests {
    use crate::runner::once::{run, Config};
    use crate::{Prng, Seed};

    fn default_prng() -> Prng {
        Prng::from_seed(Seed::from(42))
    }

    fn default_config() -> Config {
        Config {
            limit: 100.into(),
            hints_enabled: true,
            stats_enabled: false,
        }
    }

    #[test]
    fn has_error_if_test_fails() {
        let config = default_config();
        let report = run(default_prng(), &config, |_| panic!());
        assert!(report.error.is_some());
    }

    #[test]
    fn no_error_if_test_succeeds() {
        let config = default_config();
        let report = run(default_prng(), &config, |_| ());
        assert!(report.error.is_none());
    }

    #[test]
    fn no_hints_if_disabled_and_test_succeeds() {
        let config = Config {
            hints_enabled: false,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| ());
        assert!(report.hints.is_none());
    }

    #[test]
    fn no_hints_if_disabled_and_test_fails() {
        let config = Config {
            hints_enabled: false,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| panic!());
        assert!(report.hints.is_none());
    }

    #[test]
    fn has_hints_if_enabled_and_test_succeeds() {
        let config = Config {
            hints_enabled: true,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| ());
        assert!(report.hints.is_some());
    }

    #[test]
    fn has_hints_if_enabled_and_test_fails() {
        let config = Config {
            hints_enabled: true,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| panic!());
        assert!(report.hints.is_some());
    }

    #[test]
    fn no_stats_if_disabled_and_test_succeeds() {
        let config = Config {
            stats_enabled: false,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| ());
        let stats = report.stats;
        assert!(stats.is_none());
    }

    #[test]
    fn no_stats_if_disabled_and_test_fails() {
        let config = Config {
            stats_enabled: false,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| panic!());
        let stats = report.stats;
        assert!(stats.is_none());
    }

    #[test]
    fn has_stats_if_enabled_and_test_succeeds() {
        let config = Config {
            stats_enabled: true,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| ());
        let stats = report.stats;
        assert!(stats.is_some());
    }

    #[test]
    fn has_stats_if_enabled_and_test_fails() {
        let config = Config {
            stats_enabled: true,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| panic!());
        let stats = report.stats;
        assert!(stats.is_some());
    }
}
