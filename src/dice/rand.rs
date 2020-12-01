use rand::distributions::Distribution;

use crate::prelude::*;

/// Generates a value using the given [`Distribution`].
///
/// Only available if the feature `rand_full` is enabled.
pub fn from_distribution<T, D>(distribution: D) -> impl Die<T>
where
    D: Distribution<T>,
{
    dice::from_fn(move |mut fate| {
        let mut prng = fate.fork_prng();
        distribution.sample(&mut prng)
    })
}
