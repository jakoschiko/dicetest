use crate::die::adapters::{BoxedDieOnce, FlatMapDie, FlattenDie, MapDie};
use crate::die::{Fate, Limit, Prng, Seed};

/// Trait for generating a single preudorandom value of type `T`.
pub trait DieOnce<T> {
    /// Consumes the generator and generates a preudorandom value.
    ///
    /// The `Fate` is the only source of the randomness. Besides that, the generation is
    /// derterministic.
    fn roll_once(self, fate: &mut Fate) -> T;

    /// Creates a new `DieOnce` by mapping the generated values of `self`.
    ///
    /// The function `f` will be applied to the generated value of `self`. This function result
    /// is the generated value of the new `Die`.
    fn map_once<U, F>(self, f: F) -> MapDie<T, U, Self, F>
    where
        Self: Sized,
        F: FnOnce(T) -> U,
    {
        MapDie::new(self, f)
    }

    /// Creates a new `DieOnce` whose value is generated by the generated `DieOnce` of `self`.
    fn flatten_once<U>(self) -> FlattenDie<U, T, Self>
    where
        Self: Sized,
        T: DieOnce<U>,
    {
        FlattenDie::new(self)
    }

    /// Creates a new `DieOnce` similiar to `DieOnce::map_once`, except that the mapping produces
    /// a `DieOnce`.
    ///
    /// The function `f` will be applied to the generated value of `self`. This function result
    /// ia a `DieOnce` that generate the value for the new `Die`.
    ///
    /// It is semanticly equivalent to `self.map_once(f).flatten_once()`.
    fn flat_map_once<U, DU, F>(self, f: F) -> FlatMapDie<T, U, Self, DU, F>
    where
        Self: Sized,
        DU: DieOnce<U>,
        F: FnOnce(T) -> DU,
    {
        FlatMapDie::new(self, f)
    }

    /// Puts `self` behind a `Box` pointer.
    fn boxed_once<'a>(self) -> BoxedDieOnce<'a, T>
    where
        Self: Sized + 'a,
    {
        BoxedDieOnce::new(self)
    }

    /// Calls `DieOnce::roll_once` with random seed and default parameters. Useful for debugging the
    /// generator.
    fn sample_once(self) -> T
    where
        Self: Sized,
    {
        let mut prng = Prng::from_seed(Seed::random());
        let lim = Limit::default();
        let mut fate = Fate::new(&mut prng, lim);

        self.roll_once(&mut fate)
    }
}
