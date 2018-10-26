use crate::rng::Rng;
use crate::prop::Prop;
use crate::checker::EvalParams;

/// Evaluates the property one time with random seed and default `Limit`. Panics with a summary
/// of the property evaluation.
pub fn debug_prop_eval<F, P>(prop_fn: F)
where
    P: Prop,
    F: FnOnce() -> P,
{
    let eval_params = EvalParams {
        rng: Rng::random(),
        limit: Default::default(),
    };

    debug_prop_eval_with_params(eval_params, prop_fn)
}

/// Evaluates the property one time with parameters decoded from the evaluation code. Panics with
/// a summary of the property evaluation.
pub fn debug_prop_eval_with_code<F, P>(eval_code: &str, prop_fn: F)
where
    P: Prop,
    F: FnOnce() -> P,
{
    let eval_params = EvalParams::from_eval_code(eval_code)
        .expect("The test code is invalid");

    debug_prop_eval_with_params(eval_params, prop_fn)
}

/// Evaluates the property one time with the given evaluation parameters. Panics with the summary
/// of the property evaluation.
pub fn debug_prop_eval_with_params<F, P>(eval_params: EvalParams, prop_fn: F)
where
    P: Prop,
    F: FnOnce() -> P,
{
    let eval_code = eval_params.eval_code();

    let mut rng = eval_params.rng;
    let lim = eval_params.limit;
    let prop = prop_fn();
    let sample = prop.sample_with_params(&mut rng, lim);

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
