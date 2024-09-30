use crate::prelude::*;

/// Generates the given value.
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
/// assert_eq!(fate.roll(dice::just_once(42)), 42);
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct CannotBeCloned;
/// assert_eq!(fate.roll(dice::just_once(CannotBeCloned)), CannotBeCloned);
/// ```
pub fn just_once<T>(value: T) -> impl DieOnce<T> {
    dice::from_fn_once(|_| value)
}

/// Generates a clone of the given value.
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
/// assert_eq!(fate.roll(dice::just(42)), 42);
///
/// let cloning_die = dice::just(vec![0, 1, 2]);
/// for _ in 0..10 {
///     assert_eq!(fate.roll(&cloning_die), vec![0, 1, 2]);
/// }
/// ```
pub fn just<T>(value: T) -> impl Die<T>
where
    T: Clone,
{
    dice::from_fn(move |_| value.clone())
}
