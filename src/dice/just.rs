use crate::prelude::dice::*;

/// Generates the given value.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
///
/// Fate::run(&mut prng, limit, |fate| {
///     assert_eq!(dice::just_once(42).roll_once(fate), 42);
///
///     #[derive(Debug, PartialEq, Eq)]
///     struct CannotBeCloned;
///     assert_eq!(dice::just_once(CannotBeCloned).roll_once(fate), CannotBeCloned);
/// });
/// ```
pub fn just_once<T>(value: T) -> impl DieOnce<T> {
    dice::from_fn_once(|_| value)
}

/// Generates a clone of the given value.
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
///
/// Fate::run(&mut prng, limit, |fate| {
///     assert_eq!(dice::just(42).roll(fate), 42);
///
///     let cloning_die = dice::just(vec![0, 1, 2]);
///     for _ in 0..10 {
///         assert_eq!(cloning_die.roll(fate), vec![0, 1, 2]);
///     }
/// });
/// ```
pub fn just<T>(value: T) -> impl Die<T>
where
    T: Clone,
{
    dice::from_fn(move |_| value.clone())
}
