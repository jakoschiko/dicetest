use crate::prelude::dice::*;

/// Generates the next number of the `Prng` that will be passed to the generator.
///
/// # Examples
///
/// ```
/// use dicetest::prand::Seed;
/// use dicetest::prelude::dice::*;
///
/// let mut orig_prng = Prng::from_seed(Seed::random());
/// let mut prng = orig_prng.clone();
/// let fate = &mut Fate::new(&mut prng, Limit::default());
///
/// let next_number = dice::prng_next_number().roll(fate);
///
/// assert_eq!(next_number, orig_prng.next_number());
/// ```
pub fn prng_next_number() -> impl Die<u64> {
    dice::from_fn(|fate| fate.prng.next_number())
}

/// Generates a fork of the `Prng` that will be passed to the generator.
///
/// # Examples
///
/// ```
/// use dicetest::prand::Seed;
/// use dicetest::prelude::dice::*;
///
/// let mut orig_prng = Prng::from_seed(Seed::random());
/// let mut prng = orig_prng.clone();
/// let fate = &mut Fate::new(&mut prng, Limit::default());
///
/// let forked_prng = dice::prng_fork().roll(fate);
///
/// assert_eq!(forked_prng, orig_prng.fork());
/// ```
pub fn prng_fork() -> impl Die<Prng> {
    dice::from_fn(|fate| fate.prng.fork())
}
