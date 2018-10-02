use ::rng::Rng;
use ::gen::Size;
use ::prop::{Log, Eval, Prop};

/// Helper for implementing a `Prop` from a `FnOnce`.
///
/// The given `FnOnce` already implements the trait `Prop`, but using this helper may improve the
/// type inference.
pub fn from_fn_once<F>(f: F) -> impl Prop
where
    F: FnOnce(&mut Rng, Size, &mut Log) -> Eval,
{
    f
}
