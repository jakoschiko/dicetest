use rand::{self, Rng as LibRng};

use crate::util::workers;
use crate::rng::Rng;
use crate::prop::{Log, Prop};
use crate::brooder::{
    EvalParams, EvalSummary, EvalSeries,
    Params, ThreadErr, Status, Report,
    LimitSeries, Portions,
};

/// Checks the property by evaluting it several times.
pub fn brood_prop<P, F>(params: Params, prop_fn: F) -> Report
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    let seed = params.seed.unwrap_or_else(|| {
        rand::thread_rng().gen()
    });

    let mut rng = Rng::init(seed);

    let status = if params.worker_count == 0 {
        let limit_series = LimitSeries::new(
            params.start_limit,
            params.end_limit,
            params.min_passed,
        );

        let eval_series = brood_series(rng, limit_series, prop_fn.clone());

        Status::Checked(eval_series)
    } else {
        let min_passed_portions = Portions::new(
            params.min_passed,
            params.worker_count,
        );

        let funs = min_passed_portions.into_iter().map(|min_passed| {
            let worker_rng =  rng.fork();
            let limit_series = LimitSeries::new(
                params.start_limit,
                params.end_limit,
                min_passed,
            );
            let prop_fn = prop_fn.clone();
            move || brood_series(worker_rng, limit_series, prop_fn)
        }).collect();

        let joined_result = workers::run(funs, params.timeout);

        status_from_joined_result(joined_result)
    };

    let mut report = Report { seed, params, status };

    calculate_prints_if_falsified(&mut report, prop_fn);

    report
}

fn brood_series<P, F>(
    mut rng: Rng,
    limit_series: LimitSeries,
    prop_fn: F
) -> EvalSeries
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static
{
    let mut series_acc = EvalSeries::new();

    for limit in limit_series.into_iter() {
        // For performance reasons, we disable print here.
        // If the property will be falsified and all workers are
        // done, we reevalute the property with enabled print.
        let mut log = Log::with_all_disabled();

        // We clone the `Rng` to be able to reevalute the property
        let eval_rng = rng.clone();

        let prop = prop_fn();
        let eval = prop.eval(&mut log, &mut rng, limit);

        let series_next = EvalSeries::from_eval(eval, log.data().prints, move || {
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

fn calculate_prints_if_falsified<P, F>(report: &mut Report, prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    if let Status::Checked(
        EvalSeries {
            summary: EvalSummary::False {
                ref counterexample,
                ref mut prints,
            },
            ..
        }
    ) = report.status {
        let mut log = Log::with_print_enabled();
        let mut rng = counterexample.rng.clone();
        let lim = counterexample.limit;

        let prop = prop_fn();
        let _ = prop.eval(&mut log, &mut rng, lim);

        *prints = log.data().prints;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;

    use crate::prop::{Eval, Prints};
    use crate::brooder::{
        EvalParams,
        EvalSummary, EvalSeries,
        Params, Status, Report,
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

    fn expect_eval_summary_false(eval_series: EvalSeries) -> (EvalParams, Prints) {
        match eval_series.summary {
            EvalSummary::False { counterexample, prints } => (counterexample, prints),
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
                let params = Params::default()
                    .worker_count(worker_count);

                let report = brood_prop(params, move || truth);
                let eval_series = expect_status_checked(report);

                assert_eq!(0, eval_series.passed_tests)
            }
        })
    }

    #[test]
    fn full_min_passed_if_prop_evaluates_to_passed() {
        test_with_different_worker_count(|worker_count| {
            for &min_passed in &[0, 1, 2, 100] {
                let params = Params::default()
                    .worker_count(worker_count)
                    .min_passed(min_passed);

                let report = brood_prop(params, || Eval::Passed);
                let eval_series = expect_status_checked(report);

                assert_eq!(min_passed, eval_series.passed_tests)
            }
        })
    }

    #[test]
    fn contains_prints_if_prop_evaluates_to_false() {
        test_with_different_worker_count(|worker_count| {
            let params = Params::default()
                .worker_count(worker_count);

            let report = brood_prop(params, || Eval::False);
            let eval_series = expect_status_checked(report);
            let (_, prints) = expect_eval_summary_false(eval_series);

            assert!(!prints.0.is_empty())
        })
    }

    #[test]
    fn does_not_timeout_if_no_workers() {
        let params = Params::default()
            .worker_count(0)
            .timeout(Some(Duration::from_millis(10)));

        let report = brood_prop(params, || {
            thread::sleep(Duration::from_millis(100));
            Eval::True
        });

        let _ = expect_status_checked(report);
    }

    #[test]
    fn does_timeout_if_at_least_one_worker() {
        let params = Params::default()
            .worker_count(1)
            .timeout(Some(Duration::from_millis(1)));

        let report = brood_prop(params, || {
            thread::sleep(Duration::from_millis(1000));
            Eval::True
        });

        expect_status_timeout(report);
    }
}
