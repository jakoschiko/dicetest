use crate::die::adapters::{ArcDie, BoxedDie, FlatMapDie, FlattenDie, MapDie, RcDie};
use crate::die::{DieOnce, Limit};
use crate::prand::{Prng, Seed};

/// Trait for generating preudorandom values of type `T`.
///
/// The `Die` trait represents a subset of `DieOnce`. It mirrors all methods of `DieOnce` without
/// the suffix `_once`. These methods must behave in the same way. For example an implementation
/// of `Die` must produce the same value with its methods `roll` and `roll_once` if they are called
/// with the same `Prng` and `Limit`.
pub trait Die<T>: DieOnce<T> {
    /// Generates a preudorandom value.
    ///
    /// The `Prng` is the only source of the randomness. Besides that, the generation is
    /// derterministic. The `Limit` is meant as an upper size of the generated value, though
    /// it's depends on the implementation how `Limit` is interpreted.
    fn roll(&self, prng: &mut Prng, limit: Limit) -> T;

    /// Creates a new `Die` by mapping the generated values of `self`.
    ///
    /// The function `f` will be applied to the generated values of `self`. These function results
    /// are the generated values of the new `Die`.
    fn map<U, F>(self, f: F) -> MapDie<T, U, Self, F>
    where
        Self: Sized,
        F: Fn(T) -> U,
    {
        MapDie::new(self, f)
    }

    /// Creates a new `Die` whose values are generated by the generated `Die`s of `self`.
    fn flatten<U>(self) -> FlattenDie<U, T, Self>
    where
        Self: Sized,
        T: DieOnce<U>,
    {
        FlattenDie::new(self)
    }

    /// Creates a new `Die` similiar to `Die::map`, except that the mapping produces `DieOnce`s.
    ///
    /// The function `f` will be applied to the generated values of `self`. These function results
    /// are `DieOnce`s that generates the values for the new `Die`.
    ///
    /// It is semanticly equivalent to `self.map(f).flatten()`.
    fn flat_map<U, UD, F>(self, f: F) -> FlatMapDie<T, U, Self, UD, F>
    where
        Self: Sized,
        UD: DieOnce<U>,
        F: Fn(T) -> UD,
    {
        FlatMapDie::new(self, f)
    }

    /// Puts `self` behind a `Box` pointer.
    fn boxed<'a>(self) -> BoxedDie<'a, T>
    where
        Self: Sized + 'a,
    {
        BoxedDie::new(self)
    }

    /// Puts `self` behind an `Rc` pointer.
    fn rc<'a>(self) -> RcDie<'a, T>
    where
        Self: Sized + 'a,
    {
        RcDie::new(self)
    }

    /// Puts `self` behind an `Arc` pointer.
    fn arc(self) -> ArcDie<T>
    where
        Self: Sized + 'static,
    {
        ArcDie::new(self)
    }

    /// Calls `roll` with random `Seed` and default `Limit`. Useful for debugging the
    /// generator.
    fn sample(&self) -> T {
        self.sample_with_limit(Limit::default())
    }

    /// Calls `roll` with random `Seed` and the given `Limit`. Useful for debugging the
    /// generator.
    fn sample_with_limit(&self, limit: Limit) -> T {
        let mut prng = Prng::from_seed(Seed::random());

        self.roll(&mut prng, limit)
    }
}

impl<T, TD: Die<T>> DieOnce<T> for &TD {
    fn roll_once(self, prng: &mut Prng, limit: Limit) -> T {
        (*self).roll(prng, limit)
    }
}

impl<T, TD: Die<T>> Die<T> for &TD {
    fn roll(&self, prng: &mut Prng, limit: Limit) -> T {
        (**self).roll(prng, limit)
    }
}
