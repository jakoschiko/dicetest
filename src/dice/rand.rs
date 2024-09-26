use rand::distributions::Distribution;

use crate::prelude::*;

/// Generates a value using the given [`Distribution`].
///
/// Only available if the feature `rand` is enabled.
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
pub fn from_distribution<T, D>(distribution: D) -> impl Die<T>
where
    D: Distribution<T>,
{
    dice::from_fn(move |mut fate| {
        let mut prng = fate.fork_prng();
        distribution.sample(&mut prng)
    })
}
