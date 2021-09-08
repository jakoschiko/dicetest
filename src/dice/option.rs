use crate::prelude::*;

/// Generates a [`None`] or a [`Some`] that contains a value from the given generator.
/// [`None`] and [`Some`] have the same probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let some_die = dice::just_once(42);
/// let option_die = dice::option_once(some_die);
///
/// let some_or_none = fate.roll(option_die);
/// ```
pub fn option_once<T>(some_die: impl DieOnce<T>) -> impl DieOnce<Option<T>> {
    dice::one_of_die_once().two(dice::just_once(None), some_die.map_once(Some))
}

/// Generates a [`None`] or a [`Some`] that contains a value from the given generator.
/// [`None`] and [`Some`] have the same probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let some_die = dice::just(42);
/// let option_die = dice::option(some_die);
///
/// let some_or_none = fate.roll(option_die);
/// ```
pub fn option<T>(some_die: impl Die<T>) -> impl Die<Option<T>> {
    dice::one_of_die().two(dice::from_fn(|_| None), some_die.map(Some))
}

/// Generates a [`None`] or a [`Some`] that contains a value from the given generator.
/// The probabilities of [`None`] and [`Some`] depend on the given weights.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let some_die = dice::just_once(42);
/// let option_die = dice::weighted_option_once(10, (1, some_die));
///
/// let probably_none = fate.roll(option_die);
/// ```
pub fn weighted_option_once<T>(
    none_weight: u32,
    (some_weight, some_die): (u32, impl DieOnce<T>),
) -> impl DieOnce<Option<T>> {
    dice::weighted_one_of_die_once().two(
        (none_weight, dice::just_once(None)),
        (some_weight, some_die.map_once(Some)),
    )
}

/// Generates a [`None`] or a [`Some`] that contains a value from the given generator.
/// The probabilities of [`None`] and [`Some`] depend on the given weights.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let some_die = dice::just(42);
/// let option_die = dice::weighted_option(10, (1, some_die));
///
/// let probably_none = fate.roll(option_die);
/// ```
pub fn weighted_option<T>(
    none_weight: u32,
    (some_weight, some_die): (u32, impl Die<T>),
) -> impl Die<Option<T>> {
    dice::weighted_one_of_die().two(
        (none_weight, dice::from_fn(|_| None)),
        (some_weight, some_die.map(Some)),
    )
}
