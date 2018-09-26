use ::prop::Prop;
use ::checker::{EvalSeriesStatus, Params, Status, check_prop_with_params};

/// Evaluates the property several times with default parameters. Panics if the property was
/// falsified or the evaluation failed.
pub fn assert_prop<P, F>(prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    let params = Params::default();

    assert_prop_with_params(params, prop_fn)
}

/// Evaluates the property several times with default parameters and the given seed. Panics if
/// the property was falsified or the evaluation failed.
pub fn assert_prop_with_seed<P, F>(seed: u64, prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    let params = Params::default()
        .seed(Some(seed));

    assert_prop_with_params(params, prop_fn)
}

/// Evaluates the property several times with the given seed. Panics if the property was falsified
/// or the evaluation failed.
pub fn assert_prop_with_params<P, F>(params: Params, prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    let result = check_prop_with_params(params, prop_fn);

    let success = match result.status {
        Status::Checked(ref eval_series_result) => {
            match eval_series_result.status {
                EvalSeriesStatus::True => true,
                EvalSeriesStatus::Passed => true,
                _ => false,
            }
        }
        _ => false,
    };

    if !success {
        let summary = result.summary();

        panic!(summary);
    }
}
