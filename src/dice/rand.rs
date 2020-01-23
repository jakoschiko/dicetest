use crate::prelude::dice::*;

/// Generates a value using the given `Distribution` from the [crate `rand`].
///
/// [crate `rand`]: https://crates.io/crates/rand
///
/// Only available if the feature `rand` is enabled.
pub fn from_distribution<T, D>(distribution: D) -> impl Die<T>
where
    D: rand::distributions::Distribution<T>,
{
    dice::from_fn(move |fate| distribution.sample(fate.prng))
}
