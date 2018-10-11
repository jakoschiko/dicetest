use ::prop::{Log, Eval, Prop};
use ::checker::EvalParams;

/// Evaluates the property one time with parameters decoded from the evaluation code. Panics if
/// the property was falsified.
pub fn debug_prop<F, P>(eval_code: &str, prop_fn: F)
where
    P: Prop,
    F: FnOnce() -> P,
{
    let eval_params = EvalParams::from_eval_code(eval_code)
        .expect("The test code is invalid");

    debug_prop_with_params(eval_params, prop_fn)
}

/// Evaluates the property one time with the given evaluation parameters. Panics if the property
/// was falsified.
pub fn debug_prop_with_params<F, P>(eval_params: EvalParams, prop_fn: F)
where
    P: Prop,
    F: FnOnce() -> P,
{
    let mut log = Log::with_print_enabled();
    let mut rng = eval_params.rng;
    let lim = eval_params.limit;

    let prop = prop_fn();
    let eval = prop.eval(&mut log, &mut rng, lim);

    if eval == Eval::False {
        let prints = log.data().prints.pretty();
        panic!("Property was falsified with prints:\n{}\n", prints);
    }
}
