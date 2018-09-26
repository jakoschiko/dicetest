use ::gen;
use ::prop::{self, Prop};
use ::checker::{EvalParams, EvalSeriesParams, EvalSeriesStatus, EvalSeriesResult, SizeSeries};

/// Evaluates the property several times and returns the merged result of the property evaluations.
pub fn run<P, F>(params: EvalSeriesParams, prop_fn: F) -> EvalSeriesResult
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static
{
    let size_series = SizeSeries::new(params.start_size, params.end_size, params.min_passed);

    let mut rng = params.rng;

    let mut result_acc = EvalSeriesResult::new();

    for size in size_series.into_iter() {
        let prop_params = prop::Params {
            // For performance reasons, we do not create labels here.
            // If the property will be falsified and all workers are done,
            // we reevalute the property and create the labels.
            create_labels: false,
            gen_params: gen::Params { size },
        };

        // We clone the `Rng` to be able to reevalute the property
        let eval_rng = rng.clone();

        let prop = prop_fn();
        let prop_result = prop.eval(&mut rng, &prop_params);

        let result = EvalSeriesResult::from_prop_result(prop_result, move || {
            EvalParams {
                rng: eval_rng,
                gen_params: prop_params.gen_params,
            }
        });

        result_acc = result_acc.merge(result);

        match result_acc.status {
            EvalSeriesStatus::True => break,
            EvalSeriesStatus::Passed => (),
            EvalSeriesStatus::False { .. } => break,
        }
    }

    result_acc
}
