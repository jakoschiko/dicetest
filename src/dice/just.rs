use crate::prelude::dice::*;

/// Generates the given value.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// assert_eq!(dice::just_once(42).sample_once(), 42);
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct CannotBeCloned;
/// assert_eq!(dice::just_once(CannotBeCloned).sample_once(), CannotBeCloned);
/// ```
pub fn just_once<T>(value: T) -> impl DieOnce<T> {
    dice::from_fn_once(|_| value)
}

/// Generates a clone of the given value.
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// assert_eq!(dice::just(42).sample(), 42);
///
/// let cloning_die = dice::just(vec![0, 1, 2]);
/// for _ in 0..10 {
///     assert_eq!(cloning_die.sample(), vec![0, 1, 2]);
/// }
/// ```
pub fn just<T>(value: T) -> impl Die<T>
where
    T: Clone,
{
    dice::from_fn(move |_| value.clone())
}
