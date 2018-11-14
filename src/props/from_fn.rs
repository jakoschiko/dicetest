use crate::prelude::props::*;

/// Helper for implementing a `Prop` from a `FnOnce`.
///
/// The given `FnOnce` already implements the trait `Prop`, but using this helper may improve the
/// type inference.
pub fn from_fn<F>(f: F) -> impl Prop
where
    F: FnOnce(&mut Rng, Limit) -> Eval,
{
    f
}
