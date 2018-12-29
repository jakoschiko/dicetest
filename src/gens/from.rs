use crate::prelude::gens::*;

/// Helper for implementing a `GenOnce` from a `FnOnce` that takes a `Dice`.
///
/// The given `FnOnce` already implements the trait `GenOnce`, but using this helper may improve the
/// type inference.
pub fn from_fn_once<T, F>(f: F) -> impl GenOnce<T>
where
    F: FnOnce(&mut Dice) -> T,
{
    f
}

/// Helper for implementing a `Gen` from a `Fn` that takes a `Dice`.
///
/// The given `Fn` already implements the trait `Gen`, but using this helper may improve the
/// type inference.
pub fn from_fn<T, F>(f: F) -> impl Gen<T>
where
    F: Fn(&mut Dice) -> T,
{
    f
}

/// Helper for implementing a `Gen` from a `Fn` that returns a `GenOnce`.
pub fn from_gen_once_fn<T, GT, F>(f: F) -> impl Gen<T>
where
    GT: GenOnce<T>,
    F: Fn() -> GT,
{
    from_fn(move |dice| f().gen_once(dice))
}