use crate::prelude::props::*;

/// This property holds if the premise is false or the conclusion holds.
pub fn implies<P, F>(premise: bool, conclusion: F) -> impl Prop
where
    P: Prop,
    F: FnOnce() -> P,
{
    props::from_fn(move |rng, lim| {
        if premise {
            log!("Premise is false");
            Eval::True
        } else {
            let prop = conclusion();
            prop.eval(rng, lim)
        }
    })
}
