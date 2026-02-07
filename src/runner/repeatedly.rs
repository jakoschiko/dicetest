//! Provides a runner function that runs a test repeatedly.
//!
//! This runner function can be used for property testing because it tries to falsify the
//! test assertions by running the test with different seeds. If the test panics, a counterexample
//! has been found.

use std::panic::{RefUnwindSafe, UnwindSafe, catch_unwind};

use crate::hints::Hints;
use crate::runner::Error;
use crate::runner::{self, LimitSeries};
use crate::stats::Stats;
use crate::{Fate, Limit, Prng, hints};

/// An additional regression test that will be run before the random test runs.
#[derive(Debug, Clone)]
pub struct Regression {
    /// The initial state of the number generator the regression test will use for generating
    /// test data.
    pub prng: Prng,
    /// The limit for dynamic data structures the regression test will use for generating
    /// test data.
    pub limit: Limit,
}

/// The configuration for repeated test runs.
///
/// It contains parameters for both regression tests and random tests.
#[derive(Debug, Clone)]
pub struct Config {
    /// Additional regression tests that will be run before the random test runs.
    pub regressions: Vec<Regression>,
    /// The initial upper limit for the length of generated dynamic data structures
    ///
    /// It's used for the first test run. The following test runs use an
    /// interpolated limit between [`start_limit`] and [`end_limit`].
    ///
    /// [`start_limit`]: Config::start_limit
    /// [`end_limit`]: Config::end_limit
    pub start_limit: Limit,
    /// The final upper limit for the length of generated dynamic data structures.
    ///
    /// It's used for the last test run. The previous test runs use an interpolated limit
    /// between [`start_limit`] and [`end_limit`].
    ///
    /// [`start_limit`]: Config::start_limit
    /// [`end_limit`]: Config::end_limit
    pub end_limit: Limit,
    /// Defines how many times the test needs to be run without failing.
    ///
    /// The runner aborts early if a counterexample has been found.
    pub passes: u64,
    /// Defines whether the counterexample will be rerun with enabled hints. The hints will be
    /// added to the report.
    ///
    /// This parameter works only if the feature `hints` is present.
    pub hints_enabled: bool,
    /// Defines whether the stats will be enabled during the test runs. The stats will be added
    /// to the report.
    ///
    /// This parameter works only if the feature `stats` is present.
    pub stats_enabled: bool,
}

/// Contains details about a failed test run.
#[derive(Debug)]
pub struct Counterexample {
    /// The initial state of the number generator the counterexample has used for generating
    /// test data.
    pub prng: Prng,
    /// The limit for dynamic data structures the counterexample has used for generating
    /// test data.
    pub limit: Limit,
    /// The hints collected during the counterexample run.
    ///
    /// If hints are enabled, the runner tries to rerun the counterexample to collect hints.
    /// Rerunning the counterexample can fail if the test is not deterministic.
    pub hints: Option<Hints>,
    /// The error occurred during the counterexample run.
    pub error: Error,
}

/// The result of repeated test runs.
#[derive(Debug)]
pub struct Report {
    /// The number of test runs that did not fail.
    ///
    /// Both regression tests and random tests are included in this number.
    pub passes: u64,
    /// The stats collected during all test runs. It's defined if and only if stats are enabled.
    pub stats: Option<Stats>,
    /// If defined it contains the failed test run. Otherwise all test runs were successful.
    pub counterexample: Option<Counterexample>,
}

/// Runs the test repeatedly with the given configuration and different seeds.
///
/// The test will be run until the configured number of passes has been reached or a test run
/// has failed.
pub fn run<T>(prng: Prng, config: &Config, test: T) -> Report
where
    T: Fn(Fate) + UnwindSafe + RefUnwindSafe,
{
    let limit_series = LimitSeries::new(config.start_limit, config.end_limit, config.passes);

    let test_runs = || search_counterexample(&config.regressions, prng, limit_series, &test);

    let ((passes, counterexample_without_hints), stats) =
        runner::util::collect_stats(config.stats_enabled, test_runs);

    let counterexample = if config.hints_enabled {
        counterexample_without_hints
            .map(|counterexample| rerun_counterexample(counterexample, &test))
    } else {
        counterexample_without_hints
    };

    Report {
        passes,
        stats,
        counterexample,
    }
}

fn search_counterexample<T>(
    regressions: &[Regression],
    mut prng: Prng,
    limit_series: LimitSeries,
    test: &T,
) -> (u64, Option<Counterexample>)
where
    T: Fn(Fate) + UnwindSafe + RefUnwindSafe,
{
    let mut passes = 0;

    for regression in regressions {
        let test_result = catch_unwind(|| {
            let mut prng = regression.prng.clone();
            let fate = Fate::new(&mut prng, regression.limit);
            test(fate);
        });

        if let Err(err) = test_result {
            let counterexample = Counterexample {
                prng: regression.prng.clone(),
                limit: regression.limit,
                hints: None,
                error: Error(err),
            };
            return (passes, Some(counterexample));
        }

        passes += 1;
    }

    let mut limits = limit_series.into_iter();

    loop {
        let limit = match limits.next() {
            None => return (passes, None),
            Some(limit) => limit,
        };

        let prng_before_run = prng.clone();

        let test_result = catch_unwind(|| {
            let fate = Fate::new(&mut prng, limit);
            test(fate);
            prng
        });

        prng = match test_result {
            Err(err) => {
                let counterexample = Counterexample {
                    prng: prng_before_run,
                    limit,
                    hints: None,
                    error: Error(err),
                };
                return (passes, Some(counterexample));
            }
            Ok(prng_after_run) => prng_after_run,
        };

        passes += 1;
    }
}

fn rerun_counterexample<T>(counterexample: Counterexample, test: &T) -> Counterexample
where
    T: Fn(Fate) + UnwindSafe + RefUnwindSafe,
{
    let (test_result, hints) = {
        let mut prng = counterexample.prng.clone();
        let limit = counterexample.limit;
        hints::collect(|| {
            catch_unwind(move || {
                let fate = Fate::new(&mut prng, limit);
                test(fate)
            })
        })
    };

    match test_result {
        Ok(()) => counterexample,
        Err(err) => Counterexample {
            hints: Some(hints),
            error: Error(err),
            ..counterexample
        },
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::sync::atomic::{AtomicU64, Ordering};

    use crate::runner::repeatedly::{Config, run};
    use crate::{Prng, Seed, hints};

    use super::Regression;

    fn default_prng() -> Prng {
        Prng::from_seed(Seed::from(42))
    }

    fn default_config() -> Config {
        Config {
            regressions: Vec::new(),
            start_limit: 0.into(),
            end_limit: 100.into(),
            passes: 100,
            hints_enabled: true,
            stats_enabled: false,
        }
    }

    fn regression(seed: u64) -> Regression {
        let seed = Seed(seed);
        let prng = Prng::from_seed(seed);
        Regression {
            prng,
            limit: 42.into(),
        }
    }

    #[test]
    fn zero_passes_if_test_fails() {
        let config = default_config();
        let report = run(default_prng(), &config, |_| panic!());
        assert_eq!(report.passes, 0);
    }

    #[test]
    fn zero_passes_if_test_fails_with_regressions() {
        let mut config = default_config();
        config.regressions = vec![regression(123), regression(321)];
        let report = run(default_prng(), &config, |_| panic!());
        assert_eq!(report.passes, 0);
    }

    #[test]
    fn mixed_passes_if_test_fails_later() {
        let counter = AtomicU64::new(1);
        let config = default_config();
        let report = run(default_prng(), &config, |_| {
            let run = counter.fetch_add(1, Ordering::Relaxed);
            if run == 10 {
                panic!()
            }
        });
        assert_eq!(report.passes, 9);
    }

    #[test]
    fn mixed_passes_if_test_fails_later_with_regressions() {
        let counter = AtomicU64::new(1);
        let mut config = default_config();
        config.regressions = vec![regression(123), regression(321)];
        let report = run(default_prng(), &config, |_| {
            let run = counter.fetch_add(1, Ordering::Relaxed);
            if run == 10 {
                panic!()
            }
        });
        assert_eq!(report.passes, 9);
    }

    #[test]
    fn mixed_passes_if_regression_fails() {
        let counter = AtomicU64::new(1);
        let mut config = default_config();
        config.regressions = vec![regression(123), regression(321)];
        let report = run(default_prng(), &config, |_| {
            let run = counter.fetch_add(1, Ordering::Relaxed);
            if run == 2 {
                panic!()
            }
        });
        assert_eq!(report.passes, 1);
    }

    #[test]
    fn full_passes_if_test_succeeds() {
        let config = default_config();
        let report = run(default_prng(), &config, |_| ());
        assert_eq!(report.passes, config.passes);
    }

    #[test]
    fn full_passes_if_test_succeeds_with_regressions() {
        let mut config = default_config();
        config.regressions = vec![regression(123), regression(321)];
        let report = run(default_prng(), &config, |_| ());
        assert_eq!(report.passes, config.passes + 2);
    }

    #[test]
    fn has_counterproof_if_test_fails() {
        let config = default_config();
        let report = run(default_prng(), &config, |_| panic!());
        assert!(report.counterexample.is_some());
    }

    #[test]
    fn has_counterproof_if_test_fails_with_regressions() {
        let mut config = default_config();
        config.regressions = vec![regression(123), regression(321)];
        let report = run(default_prng(), &config, |_| panic!());
        assert!(report.counterexample.is_some());
    }

    #[test]
    fn no_counterproof_if_test_succeeds() {
        let config = default_config();
        let report = run(default_prng(), &config, |_| ());
        assert!(report.counterexample.is_none());
    }

    #[test]
    fn no_hints_if_disabled() {
        let config = Config {
            hints_enabled: false,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| panic!());
        let counterexample = report.counterexample.unwrap();
        assert!(counterexample.hints.is_none());
    }

    #[test]
    fn no_hints_if_enabled_but_failure_not_reproducible() {
        if cfg!(feature = "hints") {
            let config = Config {
                hints_enabled: true,
                passes: 1,
                ..default_config()
            };

            for _ in 0..10 {
                let (report, has_failed) = hints::collect(|| {
                    run(default_prng(), &config, |_| {
                        let should_fail = Seed::random().0.is_multiple_of(2);

                        hints::add(|| format!("{}", should_fail));

                        if should_fail {
                            panic!();
                        }
                    })
                });

                let failure_was_not_reproducible =
                    &has_failed.0[0].text == "true" && &has_failed.0[1].text == "false";

                if failure_was_not_reproducible {
                    let counterexample = report.counterexample.unwrap();
                    assert!(counterexample.hints.is_none());
                }
            }
        }
    }

    #[test]
    fn has_hints_if_enabled_and_test_deterministic() {
        let config = Config {
            hints_enabled: true,
            ..default_config()
        };
        let report = run(default_prng(), &config, |_| panic!());
        let counterexample = report.counterexample.unwrap();
        assert!(counterexample.hints.is_some());
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
    fn has_stats_if_enabled_test_succeeds() {
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
