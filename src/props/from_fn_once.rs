use ::rng::Rng;
use ::prop::{Params, Result, Prop};

/// Helper for implementing a `Prop` from a `FnOnce`.
///
/// The given `FnOnce` already implements the trait `Prop`, but using this helper may improve the
/// type inference.
pub fn from_fn_once<F>(f: F) -> impl Prop
where
    F: FnOnce(&mut Rng, &Params) -> Result,
{
    f
}
