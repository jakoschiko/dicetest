use crate::prop::Prop;
use crate::brooder::{EvalParams, Config, brood_prop};
use crate::asserts::Panic;

/// Checks the property using default configuration.
pub fn assert_prop<P, F>(prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    let config = Config::default();

    assert_prop_brooding(Panic::default(), config, prop_fn)
}

/// Checks the property by evaluating it serveral times using the given configuration.
pub fn assert_prop_brooding<P, F>(panic: Panic, config: Config, prop_fn: F)
where
    P: Prop + 'static,
    F: Fn() -> P + Send + Clone + 'static,
{
    let report = brood_prop(config, prop_fn);

    let should_panic = panic.should_panic_with_status(&report.status);

    if should_panic {
        panic!(report.pretty());
    }
}

/// Checks the property by evaluating it a single time using the given parameters.
pub fn assert_prop_sample<P>(panic: Panic, eval_params: EvalParams, prop: P)
where
    P: Prop,
{
    let eval_code = eval_params.eval_code();

    let mut rng = eval_params.rng;
    let lim = eval_params.limit;
    let sample = prop.sample_with_params(&mut rng, lim);

    let should_panic = panic.should_panic_with_eval(sample.eval);

    if should_panic {
        panic!(
            "\
            Debugging one property evaluation\n\
            Evaluation code: \"{}\"\n\
            {}\n\
            ",
            eval_code,
            sample.pretty(),
        );
    }
}
