use rand::distributions::Distribution;

use crate::prelude::*;

/// Generates a value using the given [`Distribution`].
///
/// Only available if the feature `rand` is enabled.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Limit, Prng};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let byte_die = dice::from_distribution(rand::distributions::Standard);
/// let byte: u8 = fate.roll(byte_die);
/// ```
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
