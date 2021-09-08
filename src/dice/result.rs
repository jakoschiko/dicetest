use crate::prelude::*;

/// Generates a [`Ok`] or a [`Err`] that contain a value from one of the given generators.
/// [`Ok`] and [`Err`] have the same probability.
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
/// let ok_die = dice::just_once(42);
/// let err_die = dice::just_once("error");
/// let result_die = dice::result_once(ok_die, err_die);
///
/// let ok_or_err = fate.roll(result_die);
/// ```
pub fn result_once<T, E>(
    ok_die: impl DieOnce<T>,
    err_die: impl DieOnce<E>,
) -> impl DieOnce<Result<T, E>> {
    dice::one_of_die_once().two(ok_die.map_once(Ok), err_die.map_once(Err))
}

/// Generates a [`Ok`] or a [`Err`] that contain a value from one of the given generators.
/// [`Ok`] and [`Err`] have the same probability.
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
/// let ok_die = dice::just(42);
/// let err_die = dice::just("error");
/// let result_die = dice::result(ok_die, err_die);
///
/// let ok_or_err = fate.roll(result_die);
/// ```
pub fn result<T, E>(ok_die: impl Die<T>, err_die: impl Die<E>) -> impl Die<Result<T, E>> {
    dice::one_of_die().two(ok_die.map(Ok), err_die.map(Err))
}

/// Generates a [`Ok`] or a [`Err`] that contain a value from one of the given generators.
/// The probabilities of [`Ok`] and [`Err`] depend on the given weights.
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
/// let ok_die = dice::just_once(42);
/// let err_die = dice::just_once("error");
/// let result_die = dice::weighted_result_once((1, ok_die), (10, err_die));
///
/// let probably_err = fate.roll(result_die);
/// ```
pub fn weighted_result_once<T, E>(
    (ok_weight, ok_die): (u32, impl DieOnce<T>),
    (err_weight, err_die): (u32, impl DieOnce<E>),
) -> impl DieOnce<Result<T, E>> {
    dice::weighted_one_of_die_once().two(
        (ok_weight, ok_die.map_once(Ok)),
        (err_weight, err_die.map_once(Err)),
    )
}

/// Generates a [`Ok`] or a [`Err`] that contain a value from one of the given generators.
/// The probabilities of [`Ok`] and [`Err`] depend on the given weights.
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
/// let ok_die = dice::just(42);
/// let err_die = dice::just("error");
/// let result_die = dice::weighted_result((1, ok_die), (10, err_die));
///
/// let probably_err = fate.roll(result_die);
/// ```
pub fn weighted_result<T, E>(
    (ok_weight, ok_die): (u32, impl Die<T>),
    (err_weight, err_die): (u32, impl Die<E>),
) -> impl Die<Result<T, E>> {
    dice::weighted_one_of_die().two((ok_weight, ok_die.map(Ok)), (err_weight, err_die.map(Err)))
}
