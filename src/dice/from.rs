use crate::prelude::*;

struct Fun<F>(F);

impl<T, F> DieOnce<T> for Fun<F>
where
    F: FnOnce(Fate) -> T,
{
    fn roll_once(self, fate: Fate) -> T {
        (self.0)(fate)
    }
}

impl<T, F> Die<T> for Fun<F>
where
    F: Fn(Fate) -> T,
{
    fn roll(&self, fate: Fate) -> T {
        (self.0)(fate)
    }
}

/// Helper for implementing a `DieOnce` from a `FnOnce` that takes a `Fate`.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Default::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let zero_or_one = fate.roll(dice::from_fn_once(|mut fate| fate.next_number() % 2));
/// assert!(zero_or_one == 0 || zero_or_one == 1);
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct CannotBeCloned;
/// let not_a_clone = fate.roll(dice::from_fn_once(|_| CannotBeCloned));
/// assert_eq!(not_a_clone, CannotBeCloned);
/// ```
pub fn from_fn_once<T, F>(f: F) -> impl DieOnce<T>
where
    F: FnOnce(Fate) -> T,
{
    Fun(f)
}

/// Helper for implementing a `Die` from a `Fn` that takes a `Fate`.
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
/// let zero_or_one = fate.roll(dice::from_fn(|mut fate| fate.next_number() % 2));
/// assert!(zero_or_one == 0 || zero_or_one == 1);
///
/// let vec = vec![0, 1, 2];
/// let cloning_die = dice::from_fn(move |_| vec.clone());
/// for _ in 0..10 {
///     assert_eq!(fate.roll(&cloning_die), vec![0, 1, 2]);
/// }
/// ```
pub fn from_fn<T, F>(f: F) -> impl Die<T>
where
    F: Fn(Fate) -> T,
{
    Fun(f)
}
