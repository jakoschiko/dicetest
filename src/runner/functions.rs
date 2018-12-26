use std::panic::{UnwindSafe, RefUnwindSafe, catch_unwind};

use rand::{self, Rng};

use crate::hints;
use crate::stats;
use crate::gen::{Prng, Dice};
use crate::runner::{LimitSeries, Run, Config, Error, Sample, Counterexample, Summary};

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

    Sample {
        run,
        hints,
        error,
    }
}

/// Runs the test repeatedly with the given configuration and different seeds.
///
/// The test will be run until the configured number of passes has been reached or a test run
/// has failed.
pub fn run_repeatedly<T>(config: Config, test: T) -> Summary
where
    T: Fn(&mut Dice) + UnwindSafe + RefUnwindSafe,
{
    let seed = config.seed.unwrap_or_else(|| {
        rand::thread_rng().gen()
    });

    let limit_series = LimitSeries::new(
        config.start_limit,
        config.end_limit,
        config.passes,
    );

    let test_runs = || search_counterexample(seed, limit_series, &test);

    let (passes, counterexample_without_hints, stats) = if config.stats_enabled {
        let ((passes, counterexample), stats) = stats::collect(test_runs);
        (passes, counterexample, Some(stats))
    } else {
        let (passes, counterexample) = test_runs();
        (passes, counterexample, None)
    };

    let counterexample =  if config.hints_enabled {
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
    test: &T
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
                let run = Run { prng: prng_before_run, limit };
                let hints = None;
                let error = Error(err);
                let counterexample = Counterexample { run, hints, error };
                break Some(counterexample)
            },
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
        Ok(()) => {
            counterexample
        }
        Err(err) => {
            Counterexample {
                run: counterexample.run,
                hints: Some(hints),
                error: Error(err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn zero_passes_if_test_fails() {
        // TODO: impl test
    }

    #[test]
    fn full_passes_if_test_succeeds() {
        // TODO: impl test
    }

    #[test]
    fn has_counterproof_if_test_fails() {
        // TODO: impl test
    }

    #[test]
    fn no_counterproof_if_test_succeeds() {
        // TODO: impl test
    }

    #[test]
    fn no_hints_if_disabled() {
        // TODO: impl test
    }

    #[test]
    fn no_hints_if_enabled_but_test_not_deterministic() {
        // TODO: impl test
    }

    #[test]
    fn has_hints_if_enabled_and_test_deteministic() {
        // TODO: impl test
    }

    #[test]
    fn no_stats_if_disabled() {
        // TODO: impl test
    }

    #[test]
    fn has_stats_if_enabled() {
        // TODO: impl test
    }
}
