use rand::{self, Rng as LibRng};

use ::util::workers;
use ::rng::Rng;
use ::prop::{self, Prop};
use ::checker::{
    EvalSeriesParams, EvalSeriesStatus, EvalSeriesResult,
    Params, ThreadErr, Status, Result,
    Portions, eval_series
};

/// Evaluates the property several times with default parameters. The returned `Result` is a
/// summary of all evaluations.
pub fn check_prop<P, F>(prop_fn: F) -> Result
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    let params = Params::default();

    check_prop_with_params(params, prop_fn)
}

/// Evaluates the property several times with default parameters and the given seed. The returned
/// `Result` is a summary of all evaluations.
pub fn check_prop_with_seed<P, F>(seed: u64, prop_fn: F) -> Result
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    let params = Params::default()
        .seed(Some(seed));

    check_prop_with_params(params, prop_fn)
}

/// Evaluates the property several times with the given parameters. The returned `Result` is a
/// summary of all evaluations.
pub fn check_prop_with_params<P, F>(params: Params, prop_fn: F) -> Result
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    let seed = params.seed.unwrap_or_else(|| {
        rand::thread_rng().gen()
    });

    let mut rng = Rng::init(seed);

    let status = if params.worker_count == 0 {
        let params = EvalSeriesParams {
            rng,
            start_size: params.start_size,
            end_size: params.end_size,
            min_passed: params.min_passed,
        };

        let eval_series_result = eval_series::run(params, prop_fn.clone());

        Status::Checked(eval_series_result)
    } else {
        let portions = Portions {
            total: params.min_passed,
            count: params.worker_count,
        };

        let funs = portions.into_iter().map(|portion| {
            let eval_series_params = EvalSeriesParams {
                rng: rng.fork(),
                start_size: params.start_size,
                end_size: params.end_size,
                min_passed: portion,
            };
            let prop_fn = prop_fn.clone();
            move || eval_series::run(eval_series_params, prop_fn)
        }).collect();

        let joined_result = workers::run(funs, params.timeout);

        status_from_joined_result(joined_result)
    };

    let mut result = Result { seed, params, status };

    calculate_labels_if_falsified(&mut result, prop_fn);

    result
}

fn status_from_joined_result(joined_result: workers::JoinedResult<EvalSeriesResult>) -> Status {
    if joined_result.timeout {
        Status::Timeout
    } else {
        match joined_result.oks_or_first_err() {
            Err(err) => {
                let thread_err = ThreadErr::new(err);
                Status::Panic(thread_err)
            }
            Ok(eval_series_results) => {
                let eval_series_result = {
                    let mut iter = eval_series_results.into_iter();
                    let first = iter.next().unwrap();
                    iter.fold(first, |acc, next| acc.merge(next))
                };
                Status::Checked(eval_series_result)
            }
        }
    }
}

fn calculate_labels_if_falsified<P, F>(result: &mut Result, prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    if let Status::Checked(
        EvalSeriesResult {
            status: EvalSeriesStatus::False {
                ref counterexample,
                ref mut labels,
            },
            ..
        }
    ) = result.status {
        let mut rng = counterexample.rng.clone();
        let prop_params = prop::Params {
            create_labels: true,
            gen_params: counterexample.gen_params.clone(),
        };

        let prop = prop_fn();
        let mut prop_result = prop.eval(&mut rng, &prop_params);

        *labels = prop_result.labels;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;

    use ::prop::{self, Labels};
    use ::checker::{
        EvalParams,
        EvalSeriesStatus, EvalSeriesResult,
        Params, Status, Result,
        check_prop_with_params
    };

    fn expect_status_checked(result: Result) -> EvalSeriesResult {
        match result.status {
            Status::Checked(eval_series_result) => eval_series_result,
            unexpected => panic!("Expecting Status::Checked, but got {:?}", unexpected),
        }
    }

    fn expect_status_timeout(result: Result) {
        match result.status {
            Status::Timeout => (),
            unexpected => panic!("Expecting Status::Timeout, but got {:?}", unexpected),
        }
    }

    fn expect_eval_series_status_false(result: EvalSeriesResult) -> (EvalParams, Labels) {
        match result.status {
            EvalSeriesStatus::False { counterexample, labels } => (counterexample, labels),
            unexpected => panic!("Expecting EvalSeriesStatus::False, but got {:?}", unexpected),
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
            for &truth in &[prop::Status::True, prop::Status::False] {
                let params = Params::default()
                    .worker_count(worker_count);

                let result = check_prop_with_params(params, move || truth);
                let eval_series_result = expect_status_checked(result);

                assert_eq!(0, eval_series_result.passed_tests)
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

                let result = check_prop_with_params(params, || prop::Status::Passed);
                let eval_series_result = expect_status_checked(result);

                assert_eq!(min_passed, eval_series_result.passed_tests)
            }
        })
    }

    #[test]
    fn contains_labels_if_prop_evaluates_to_false() {
        test_with_different_worker_count(|worker_count| {
            let params = Params::default()
                .worker_count(worker_count);

            let result = check_prop_with_params(params, || prop::Status::False);
            let eval_series_result = expect_status_checked(result);
            let (_, labels) = expect_eval_series_status_false(eval_series_result);

            assert!(!labels.is_empty())
        })
    }

    #[test]
    fn does_not_timeout_if_no_workers() {
        let params = Params::default()
            .worker_count(0)
            .timeout(Some(Duration::from_millis(10)));

        let result = check_prop_with_params(params, || {
            thread::sleep(Duration::from_millis(100));
            prop::Status::True
        });

        let _ = expect_status_checked(result);
    }

    #[test]
    fn does_timeout_if_at_least_one_worker() {
        let params = Params::default()
            .worker_count(1)
            .timeout(Some(Duration::from_millis(1)));

        let result = check_prop_with_params(params, || {
            thread::sleep(Duration::from_millis(1000));
            prop::Status::True
        });

        expect_status_timeout(result);
    }
}
