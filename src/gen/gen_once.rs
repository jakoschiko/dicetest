use crate::rng::Rng;
use crate::gen::Limit;
use crate::gen::adapters::{MapGenOnce, FlattenGenOnce, FlatMapGenOnce, DynGenOnce};

/// Trait for generating a single random value of type `T`.
pub trait GenOnce<T> {
    /// Consumes the generator and generates a random value.
    ///
    /// The `Rng` is the only source of the randomness. Besides that, the generation is
    /// derterministic.
    fn gen_once(self, &mut Rng, Limit) -> T;

    /// Creates a new `GenOnce` by mapping the generated values of `self`.
    ///
    /// The function `f` will be applied to the generated value of `self`. This function result
    /// is the generated value of the new `Gen`.
    fn map_once<U, F>(self, f: F) -> MapGenOnce<T, U, Self, F>
    where
        Self: Sized,
        F: FnOnce(T) -> U,
    {
        MapGenOnce::new(self, f)
    }

    /// Creates a new `GenOnce` whose value is generated by the generated `GenOnce` of `self`.
    fn flatten_once<U>(self) -> FlattenGenOnce<U, T, Self>
    where
        Self: Sized,
        T: GenOnce<U>
    {
        FlattenGenOnce::new(self)
    }

    /// Creates a new `GenOnce` similiar to `GenOnce::map_once`, except that the mapping produces
    /// a `GenOnce`.
    ///
    /// The function `f` will be applied to the generated value of `self`. This function result
    /// ia a `GenOnce` that generate the value for the new `Gen`.
    ///
    /// It is semanticly equivalent to `self.map_once(f).flatten_once()`.
    fn flat_map_once<U, GU, F>(self, f: F) -> FlatMapGenOnce<T, U, Self, GU, F>
    where
        Self: Sized,
        GU: GenOnce<U>,
        F: FnOnce(T) -> GU,
    {
        FlatMapGenOnce::new(self, f)
    }

    /// Puts `self` behind a pointer.
    fn dyn_once<'a>(self) -> DynGenOnce<'a, T>
    where
        Self: Sized + 'a,
    {
        DynGenOnce::new(self)
    }

    /// Calls `GenOnce::gen_once` with random seed and default parameters. Useful for debugging the
    /// generator.
    fn sample_once(self) -> T
    where
        Self: Sized,
    {
        let mut rng = Rng::random();
        let lim = Limit::default();

        self.gen_once(&mut rng, lim)
    }
}

impl<T, F> GenOnce<T> for F
where
    F: FnOnce(&mut Rng, Limit) -> T,
{
    fn gen_once(self, rng: &mut Rng, lim: Limit) -> T {
        self(rng, lim)
    }
}
