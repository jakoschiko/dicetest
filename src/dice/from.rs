use crate::prelude::dice::*;

struct Fun<F>(F);

impl<T, F> DieOnce<T> for Fun<F>
where
    F: FnOnce(&mut Fate) -> T,
{
    fn roll_once(self, fate: &mut Fate) -> T {
        (self.0)(fate)
    }
}

impl<T, F> Die<T> for Fun<F>
where
    F: Fn(&mut Fate) -> T,
{
    fn roll(&self, fate: &mut Fate) -> T {
        (self.0)(fate)
    }
}

/// Helper for implementing a `DieOnce` from a `FnOnce` that takes a `Fate`.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Default::default();
///
/// Fate::run(&mut prng, limit, |fate| {
///     let zero_or_one = dice::from_fn_once(|fate| fate.next_number() % 2).roll_once(fate);
///     assert!(zero_or_one == 0 || zero_or_one == 1);
///
///     #[derive(Debug, PartialEq, Eq)]
///     struct CannotBeCloned;
///     let not_a_clone = dice::from_fn_once(|_| CannotBeCloned).roll_once(fate);
///     assert_eq!(not_a_clone, CannotBeCloned);
/// });
/// ```
pub fn from_fn_once<T, F>(f: F) -> impl DieOnce<T>
where
    F: FnOnce(&mut Fate) -> T,
{
    Fun(f)
}

/// Helper for implementing a `Die` from a `Fn` that takes a `Fate`.
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
///     let zero_or_one = dice::from_fn(|fate| fate.next_number() % 2).roll(fate);
///     assert!(zero_or_one == 0 || zero_or_one == 1);
///
///     let vec = vec![0, 1, 2];
///     let cloning_die = dice::from_fn(move |_| vec.clone());
///     for _ in 0..10 {
///         assert_eq!(cloning_die.roll(fate), vec![0, 1, 2]);
///     }
/// });
/// ```
pub fn from_fn<T, F>(f: F) -> impl Die<T>
where
    F: Fn(&mut Fate) -> T,
{
    Fun(f)
}
