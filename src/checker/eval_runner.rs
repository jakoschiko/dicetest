use ::prop::{Log, Prop};
use ::rng::Rng;
use ::checker::{EvalParams, EvalSummary, EvalSeries, SizeSeries};

/// Evaluates the property several times and returns the results.
pub fn run<P, F>(
    mut rng: Rng,
    size_series: SizeSeries,
    prop_fn: F
) -> EvalSeries
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static
{
    let mut series_acc = EvalSeries::new();

    for size in size_series.into_iter() {
        // For performance reasons, we disable print here.
        // If the property will be falsified and all workers are
        // done, we reevalute the property with enabled print.
        let mut log = Log::with_all_disabled();

        // We clone the `Rng` to be able to reevalute the property
        let eval_rng = rng.clone();

        let prop = prop_fn();
        let eval = prop.eval(&mut rng, size, &mut log);

        let series_next = EvalSeries::from_eval(eval, log.data().prints, move || {
            EvalParams {
                rng: eval_rng,
                size,
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
