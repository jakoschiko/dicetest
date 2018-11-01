use std::fmt::Debug;
use crate::prelude::props::*;

/// If the result is `Ok`, passes its value to the property function. Else this property is
/// falsified.
pub fn result_ok<T, E, P, F>(result: Result<T, E>, f: F) -> impl Prop
where
    E: Debug,
    P: Prop,
    F: FnOnce(T) -> P,
{
    props::from_fn(move |log, rng, lim| {
        match result {
            Ok(value) => {
                let prop = f(value);
                prop.eval(log, rng, lim)
            },
            Err(err) => {
                log.print(|| format!("Expects Result::Ok, but got Result::Err with: {:?}", err));
                Eval::False
            },
        }
    })
}

/// If the result is `Err`, passes its error to the property function. Else this property is
/// falsified.
pub fn result_err<T, E, P, F>(result: Result<T, E>, f: F) -> impl Prop
where
    T: Debug,
    P: Prop,
    F: FnOnce(E) -> P,
{
    props::from_fn(move |log, rng, lim| {
        match result {
            Ok(value) => {
                log.print(|| format!("Expects Result::Err, but got Result::Ok with: {:?}", value));
                Eval::False
            },
            Err(err) => {
                let prop = f(err);
                prop.eval(log, rng, lim)
            },
        }
    })
}
