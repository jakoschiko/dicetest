use ::prop::{self, Prop, Status};
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
    let mut rng = eval_params.rng;
    let params = prop::Params {
        create_labels: true,
        gen_params: eval_params.gen_params,
    };

    let prop = prop_fn();
    let result = prop.eval(&mut rng, &params);

    if result.status == Status::False {
        let labels = result.labels.pretty_labels();
        panic!("Property was falsified with labels:\n{}\n", labels);
    }
}
