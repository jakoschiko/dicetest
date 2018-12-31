use crate::gen::adapters::{BoxedGenOnce, FlatMapGen, FlattenGen, MapGen};
use crate::gen::{Dice, Limit, Prng};

/// Trait for generating a single random value of type `T`.
pub trait GenOnce<T> {
    /// Consumes the generator and generates a random value.
    ///
    /// The `Dice` is the only source of the randomness. Besides that, the generation is
    /// derterministic.
    fn gen_once(self, dice: &mut Dice) -> T;

    /// Creates a new `GenOnce` by mapping the generated values of `self`.
    ///
    /// The function `f` will be applied to the generated value of `self`. This function result
    /// is the generated value of the new `Gen`.
    fn map_once<U, F>(self, f: F) -> MapGen<T, U, Self, F>
    where
        Self: Sized,
        F: FnOnce(T) -> U,
    {
        MapGen::new(self, f)
    }

    /// Creates a new `GenOnce` whose value is generated by the generated `GenOnce` of `self`.
    fn flatten_once<U>(self) -> FlattenGen<U, T, Self>
    where
        Self: Sized,
        T: GenOnce<U>,
    {
        FlattenGen::new(self)
    }

    /// Creates a new `GenOnce` similiar to `GenOnce::map_once`, except that the mapping produces
    /// a `GenOnce`.
    ///
    /// The function `f` will be applied to the generated value of `self`. This function result
    /// ia a `GenOnce` that generate the value for the new `Gen`.
    ///
    /// It is semanticly equivalent to `self.map_once(f).flatten_once()`.
    fn flat_map_once<U, GU, F>(self, f: F) -> FlatMapGen<T, U, Self, GU, F>
    where
        Self: Sized,
        GU: GenOnce<U>,
        F: FnOnce(T) -> GU,
    {
        FlatMapGen::new(self, f)
    }

    /// Puts `self` behind a `Box` pointer.
    fn boxed_once<'a>(self) -> BoxedGenOnce<'a, T>
    where
        Self: Sized + 'a,
    {
        BoxedGenOnce::new(self)
    }

    /// Calls `GenOnce::gen_once` with random seed and default parameters. Useful for debugging the
    /// generator.
    fn sample_once(self) -> T
    where
        Self: Sized,
    {
        let mut prng = Prng::random();
        let lim = Limit::default();
        let mut dice = Dice::new(&mut prng, lim);

        self.gen_once(&mut dice)
    }
}
