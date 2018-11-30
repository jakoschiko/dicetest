use std::panic::{UnwindSafe, catch_unwind};
use rand::{self, Rng as LibRng};

use crate::util::workers;
use crate::logger::{self, Messages};
use crate::counter::{self, Stats};
use crate::rng::Rng;
use crate::gen::Dice;
use crate::prop::Prop;
use crate::brooder::{
    EvalParams, EvalSummary, EvalSeries,
    Config, ThreadErr, Status, Report,
    LimitSeries, Portions,
};

/// Checks the property by evaluting it several times.
pub fn brood_prop<P, F>(config: Config, prop_fn: F) -> Report
where
    P: Prop + 'static,
    F: Fn() -> P + Clone + Send + UnwindSafe + 'static,
{
    let counter_enabled = config.counter_enabled;

    let seed = config.seed.unwrap_or_else(|| {
        rand::thread_rng().gen()
    });

    let mut rng = Rng::init(seed);

    let status = if config.worker_count == 0 {
        let limit_series = LimitSeries::new(
            config.start_limit,
            config.end_limit,
            config.min_passed,
        );

        let prop_fn = prop_fn.clone();
        let result = catch_unwind(move || {
            brood_series(rng, limit_series, counter_enabled, prop_fn)
        });

        match result {
            Ok(eval_series) => Status::Checked(eval_series),
            Err(err) => {
                let thread_err = ThreadErr::new(err);
                Status::Panic(thread_err)
            }
        }
    } else {
        let min_passed_portions = Portions::new(
            config.min_passed,
            config.worker_count,
        );

        let funs = min_passed_portions.into_iter().map(|min_passed| {
            let worker_rng =  rng.fork();
            let limit_series = LimitSeries::new(
                config.start_limit,
                config.end_limit,
                min_passed,
            );
            let prop_fn = prop_fn.clone();
            move || brood_series(worker_rng, limit_series, counter_enabled, prop_fn)
        }).collect();

        let joined_result = workers::run(funs, config.timeout);

        status_from_joined_result(joined_result)
    };

    let mut report = Report { seed, config, status };

    collect_log_messages_if_falsified(&mut report, prop_fn);

    report
}

fn brood_series<P, F>(
    mut rng: Rng,
    limit_series: LimitSeries,
    counter_enabled: bool,
    prop_fn: F
) -> EvalSeries
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static
{
    let mut series_acc = EvalSeries::new();

    for limit in limit_series.into_iter() {
        // For performance reasons, we do not enable the logger here.
        // If we need the log messages of the counterexample, we will
        // reevaluate the property afterwards.
        let messages = Messages::new();

        // We clone the `Rng` to be able to reevalute the property
        let eval_rng = rng.clone();

        let mut dice = Dice::new(&mut rng, limit);

        let (eval, stats) = {
            let mut eval = || {
                let prop = prop_fn();
                prop.eval(&mut dice)
            };
            if !counter_enabled { (eval(), Stats::new()) }
            else { counter::collect_stats(eval) }
        };

        let series_next = EvalSeries::from_eval(eval, messages, stats, move || {
            EvalParams {
                rng: eval_rng,
                limit,
            }
        });

        series_acc = series_acc.merge(series_next);

        match series_acc.summary {
            EvalSummary::True => break,
            EvalSummary::Passed => (),
            EvalSummary::False { .. } => break,
        }
    }

    series_acc
}

fn status_from_joined_result(joined_result: workers::JoinedResult<EvalSeries>) -> Status {
    if joined_result.timeout {
        Status::Timeout
    } else {
        match joined_result.oks_or_first_err() {
            Err(err) => {
                let thread_err = ThreadErr::new(err);
                Status::Panic(thread_err)
            }
            Ok(eval_serieses) => {
                let eval_series = {
                    let mut iter = eval_serieses.into_iter();
                    // We know that there is at least one worker
                    let first = iter.next().unwrap();
                    iter.fold(first, |acc, next| acc.merge(next))
                };
                Status::Checked(eval_series)
            }
        }
    }
}

fn collect_log_messages_if_falsified<P, F>(report: &mut Report, prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    if let Status::Checked(
        EvalSeries {
            summary: EvalSummary::False {
                ref counterexample,
                ref mut messages,
            },
            ..
        }
    ) = report.status {
        let mut rng = counterexample.rng.clone();
        let limit = counterexample.limit;
        let mut dice = Dice::new(&mut rng, limit);

        let (_, counterexample_messages) = logger::collect_messages(|| {
            let prop = prop_fn();
            prop.eval(&mut dice)
        });

        *messages = counterexample_messages;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;

    use crate::logger::Messages;
    use crate::counter;
    use crate::prop::Eval;
    use crate::brooder::{
        EvalParams,
        EvalSummary, EvalSeries,
        Config, Status, Report,
        brood_prop
    };

    fn expect_status_checked(report: Report) -> EvalSeries {
        match report.status {
            Status::Checked(eval_series) => eval_series,
            unexpected => panic!("Expecting Status::Checked, but got {:?}", unexpected),
        }
    }

    fn expect_status_timeout(report: Report) {
        match report.status {
            Status::Timeout => (),
            unexpected => panic!("Expecting Status::Timeout, but got {:?}", unexpected),
        }
    }

    fn expect_eval_summary_false(eval_series: EvalSeries) -> (EvalParams, Messages) {
        match eval_series.summary {
            EvalSummary::False { counterexample, messages } => (counterexample, messages),
            unexpected => panic!("Expecting EvalSummary::False, but got {:?}", unexpected),
        }
    }

    fn test_with_different_worker_count(test: impl Fn(u64)) {
        for &worker_count in &[0, 1, 10] {
            test(worker_count)
        }
    }

    #[test]
    fn no_passed_if_prop_evaluates_to_true_or_false() {
        test_with_different_worker_count(|worker_count| {
            for &truth in &[Eval::True, Eval::False] {
                let config = Config::default()
                    .worker_count(worker_count);

                let report = brood_prop(config, move || truth);
                let eval_series = expect_status_checked(report);

                assert_eq!(0, eval_series.passed_tests)
            }
        })
    }

    #[test]
    fn full_min_passed_if_prop_evaluates_to_passed() {
        test_with_different_worker_count(|worker_count| {
            for &min_passed in &[0, 1, 2, 100] {
                let config = Config::default()
                    .worker_count(worker_count)
                    .min_passed(min_passed);

                let report = brood_prop(config, || Eval::Passed);
                let eval_series = expect_status_checked(report);

                assert_eq!(min_passed, eval_series.passed_tests)
            }
        })
    }

    #[test]
    fn report_contains_counterproof_log_messages() {
        test_with_different_worker_count(|worker_count| {
            let config = Config::default()
                .worker_count(worker_count);

            let report = brood_prop(config, || Eval::False);
            let eval_series = expect_status_checked(report);
            let (_, messages) = expect_eval_summary_false(eval_series);
            let disabled_logger = cfg!(feature = "disabled_logger");

            assert!(disabled_logger || !messages.0.is_empty())
        })
    }

    fn check_stats_in_report(counter_enabled: bool) {
        test_with_different_worker_count(|worker_count| {
            let config = Config::default()
                .worker_count(worker_count)
                .counter_enabled(counter_enabled);

            let report = brood_prop(config, || {
                counter::count("foo", || "bar".to_string());
                Eval::False
            });

            let eval_series = expect_status_checked(report);
            let stats = eval_series.stats;
            let disabled_counter = cfg!(feature = "disabled_counter");

            assert!(disabled_counter || counter_enabled != stats.0.is_empty())
        })
    }

    #[test]
    fn report_contains_stats_if_enabled() {
        check_stats_in_report(true)
    }

    #[test]
    fn report_does_not_contain_stats_if_disabled() {
        check_stats_in_report(false)
    }

    #[test]
    fn does_not_timeout_if_no_workers() {
        let config = Config::default()
            .worker_count(0)
            .timeout(Some(Duration::from_millis(10)));

        let report = brood_prop(config, || {
            thread::sleep(Duration::from_millis(100));
            Eval::True
        });

        let _ = expect_status_checked(report);
    }

    #[test]
    fn does_timeout_if_at_least_one_worker() {
        let config = Config::default()
            .worker_count(1)
            .timeout(Some(Duration::from_millis(1)));

        let report = brood_prop(config, || {
            thread::sleep(Duration::from_millis(1000));
            Eval::True
        });

        expect_status_timeout(report);
    }
}
