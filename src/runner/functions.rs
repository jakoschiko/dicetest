use std::panic::{catch_unwind, RefUnwindSafe, UnwindSafe};

use rand::{self, Rng};

use crate::gen::{Dice, Prng};
use crate::hints;
use crate::runner::{Config, Counterexample, Error, LimitSeries, Run, Sample, Summary};
use crate::stats;

/// Runs the test once with the given configuration.
pub fn run_once<T>(run: Run, test: T) -> Sample
where
    T: FnOnce(&mut Dice) + UnwindSafe,
{
    let (test_result, hints) = {
        let mut prng = run.prng.clone();
        let limit = run.limit;
        hints::collect(|| {
            catch_unwind(move || {
                let mut dice = Dice::new(&mut prng, limit);
                test(&mut dice)
            })
        })
    };

    let error = test_result.err().map(Error);

    Sample { run, hints, error }
}

/// Runs the test repeatedly with the given configuration and different seeds.
///
/// The test will be run until the configured number of passes has been reached or a test run
/// has failed.
pub fn run_repeatedly<T>(config: Config, test: T) -> Summary
where
    T: Fn(&mut Dice) + UnwindSafe + RefUnwindSafe,
{
    let seed = config.seed.unwrap_or_else(|| rand::thread_rng().gen());

    let limit_series = LimitSeries::new(config.start_limit, config.end_limit, config.passes);

    let test_runs = || search_counterexample(seed, limit_series, &test);

    let (passes, counterexample_without_hints, stats) = if config.stats_enabled {
        let ((passes, counterexample), stats) = stats::collect(test_runs);
        (passes, counterexample, Some(stats))
    } else {
        let (passes, counterexample) = test_runs();
        (passes, counterexample, None)
    };

    let counterexample = if config.hints_enabled {
        counterexample_without_hints
            .map(|counterexample| rerun_counterexample(counterexample, &test))
    } else {
        counterexample_without_hints
    };

    Summary {
        config,
        seed,
        passes,
        stats,
        counterexample,
    }
}

fn search_counterexample<T>(
    seed: u64,
    limit_series: LimitSeries,
    test: &T,
) -> (u64, Option<Counterexample>)
where
    T: Fn(&mut Dice) + UnwindSafe + RefUnwindSafe,
{
    let mut passes = 0;
    let mut prng = Prng::init(seed);
    let mut limits = limit_series.into_iter();

    let counterexample = loop {
        let limit = match limits.next() {
            None => break None,
            Some(limit) => limit,
        };

        let prng_before_run = prng.clone();

        let test_result = catch_unwind(|| {
            {
                let mut dice = Dice::new(&mut prng, limit);
                test(&mut dice);
            }
            prng
        });

        prng = match test_result {
            Err(err) => {
                let run = Run {
                    prng: prng_before_run,
                    limit,
                };
                let hints = None;
                let error = Error(err);
                let counterexample = Counterexample { run, hints, error };
                break Some(counterexample);
            }
            Ok(prng_after_run) => prng_after_run,
        };

        passes += 1;
    };

    (passes, counterexample)
}

fn rerun_counterexample<T>(counterexample: Counterexample, test: &T) -> Counterexample
where
    T: Fn(&mut Dice) + UnwindSafe + RefUnwindSafe,
{
    let (test_result, hints) = {
        let mut prng = counterexample.run.prng.clone();
        let limit = counterexample.run.limit;
        hints::collect(|| {
            catch_unwind(move || {
                let mut dice = Dice::new(&mut prng, limit);
                test(&mut dice)
            })
        })
    };

    match test_result {
        Ok(()) => counterexample,
        Err(err) => Counterexample {
            run: counterexample.run,
            hints: Some(hints),
            error: Error(err),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::gen::Prng;
    use crate::hints;
    use crate::runner::{self, Config};

    #[test]
    fn zero_passes_if_test_fails() {
        let summary = runner::run_repeatedly(Config::default(), |_| panic!());
        assert_eq!(summary.passes, 0);
    }

    #[test]
    fn full_passes_if_test_succeeds() {
        let config = Config::default();
        let summary = runner::run_repeatedly(config.clone(), |_| ());
        assert_eq!(summary.passes, config.passes);
    }

    #[test]
    fn has_counterproof_if_test_fails() {
        let summary = runner::run_repeatedly(Config::default(), |_| panic!());
        assert!(summary.counterexample.is_some());
    }

    #[test]
    fn no_counterproof_if_test_succeeds() {
        let summary = runner::run_repeatedly(Config::default(), |_| ());
        assert!(summary.counterexample.is_none());
    }

    #[test]
    fn no_hints_if_disabled() {
        let config = Config::default().with_hints_enabled(false);
        let summary = runner::run_repeatedly(config, |_| panic!());
        let counterexample = summary.counterexample.unwrap();
        assert!(counterexample.hints.is_none());
    }

    #[test]
    fn no_hints_if_enabled_but_failure_not_reproduceable() {
        for _ in 0..10 {
            let config = Config::default().with_hints_enabled(true).with_passes(1);
            let (summary, has_failed) = hints::collect(|| {
                runner::run_repeatedly(config, |_| {
                    let should_fail = Prng::random().next_number() % 2 == 0;

                    hints::add(|| format!("{}", should_fail));

                    if should_fail {
                        panic!();
                    }
                })
            });

            let failure_was_not_reproduceable =
                &has_failed.0[0].text == "true" && &has_failed.0[1].text == "false";

            if failure_was_not_reproduceable {
                let counterexample = summary.counterexample.unwrap();
                assert!(counterexample.hints.is_none());
            }
        }
    }

    #[test]
    fn has_hints_if_enabled_and_test_deteministic() {
        let config = Config::default().with_hints_enabled(true);
        let summary = runner::run_repeatedly(config, |_| panic!());
        let counterexample = summary.counterexample.unwrap();
        assert!(counterexample.hints.is_some());
    }

    #[test]
    fn no_stats_if_disabled() {
        let config = Config::default().with_stats_enabled(false);
        let summary = runner::run_repeatedly(config, |_| ());
        let stats = summary.stats;
        assert!(stats.is_none());
    }

    #[test]
    fn has_stats_if_enabled() {
        let config = Config::default().with_stats_enabled(true);
        let summary = runner::run_repeatedly(config, |_| ());
        let stats = summary.stats;
        assert!(stats.is_some());
    }
}
