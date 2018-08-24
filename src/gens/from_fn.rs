use ::rng::Rng;
use ::gen::{Params, GenOnce, Gen};

/// Helper for implementing a `GenOnce` from a `FnOnce`.
///
/// The given `FnOnce` already implements the trait `GenOnce`, but this helper may improve the
/// type inference.
pub fn from_fn_once<T, F>(f: F) -> impl GenOnce<T>
where
    F: FnOnce(&mut Rng, &Params) -> T,
{
    f
}

/// Helper for implementing a `Gen` from a `Fn`.
///
/// The given `Fn` already implements the trait `Gen`, but this helper may improve the
/// type inference.
pub fn from_fn<T, F>(f: F) -> impl Gen<T>
where
    F: Fn(&mut Rng, &Params) -> T,
{
    f
}
