use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Params, GenOnce, Gen};

/// Default implementation for `Gen::map`.
pub struct GenMap<T, U, G, F>
where
    G: Gen<T>,
    F: Fn(T) -> U,
{
    g: G,
    f: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

impl<T, U, G, F> GenMap<T, U, G, F>
where
    G: Gen<T>,
    F: Fn(T) -> U,
{
    pub fn new(g: G, f: F) -> Self {
        GenMap {
            g,
            f,
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<T, U, G, F> GenOnce<U> for GenMap<T, U, G, F>
where
    G: Gen<T>,
    F: Fn(T) -> U,
{
    fn gen_once(self, rng: &mut Rng, params: &Params) -> U {
        self.gen(rng, params)
    }
}

impl<T, U, G, F> Gen<U> for GenMap<T, U, G, F>
where
    G: Gen<T>,
    F: Fn(T) -> U,
{
    fn gen(&self, rng: &mut Rng, params: &Params) -> U {
        let g = &self.g;
        let f = &self.f;

        let t = g.gen(rng, params);
        let u = f(t);

        u
    }
}
