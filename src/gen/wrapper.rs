use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Size, GenOnce, Gen};

/// Helper for hiding `GenOnce` and `Gen` implementations behind a common pointer type.
pub trait Wrapper<T> {
    /// Wrapper  for `GenOnce::gen_once`. Takes a mutable reference to `self` because
    /// `GenOnceWrapper` will consume `self`. Hence this method may panic if called twice.
    fn gen_once(&mut self, &mut Rng, Size) -> T;
    // Wrapper for `Gen::gen`. It's not implemented for `GenOnceWrapper`. Hence this method may
    // panic.
    fn gen(&self, &mut Rng, Size) -> T;
}

/// Implements `Wrapper` for `GenOnce`.
pub struct GenOnceWrapper<T, G>
where
    G: GenOnce<T>,
{
    gen: Option<G>,
    _t: PhantomData<T>,
}

impl<T, G> GenOnceWrapper<T, G>
where
    G: GenOnce<T>,
{
    pub fn new(gen: G) -> Self {
        GenOnceWrapper {
            gen: Some(gen),
            _t: PhantomData,
        }
    }
}

impl<T, G> Wrapper<T> for GenOnceWrapper<T, G>
where
    G: GenOnce<T>,
{
    fn gen_once(&mut self, rng: &mut Rng, size: Size) -> T {
        let gen = self.gen.take().expect("GenOnceWrapper::gen_once should not be called twice");
        gen.gen_once(rng, size)
    }

    fn gen(&self, _rng: &mut Rng, _size: Size) -> T {
        panic!("GenOnceWrapper::gen should not be called");
    }
}

/// Implements `Wrapper` for `Gen`.
pub struct GenWrapper<T, G>
where
    G: Gen<T>,
{
    gen: G,
    _t: PhantomData<T>,
}

impl<T, G> GenWrapper<T, G>
where
    G: Gen<T>,
{
    pub fn new(gen: G) -> Self {
        GenWrapper {
            gen,
            _t: PhantomData,
        }
    }
}

impl<T, G> Wrapper<T> for GenWrapper<T, G>
where
    G: Gen<T>,
{
    fn gen_once(&mut self, rng: &mut Rng, size: Size) -> T {
        self.gen(rng, size)
    }

    fn gen(&self, rng: &mut Rng, size: Size) -> T {
        self.gen.gen(rng, size)
    }
}
