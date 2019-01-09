use std::marker::PhantomData;

use crate::gen::{Fate, Gen, GenOnce};

/// Adapter for `GenOnce::map_once` and `Gen::map`.
pub struct MapGen<T, U, G, F> {
    g: G,
    f: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

impl<T, U, G, F> MapGen<T, U, G, F> {
    pub fn new(g: G, f: F) -> Self {
        MapGen {
            g,
            f,
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<T, U, G, F> GenOnce<U> for MapGen<T, U, G, F>
where
    G: GenOnce<T>,
    F: FnOnce(T) -> U,
{
    fn gen_once(self, fate: &mut Fate) -> U {
        let g = self.g;
        let f = self.f;

        let t = g.gen_once(fate);
        f(t)
    }
}

impl<T, U, G, F> Gen<U> for MapGen<T, U, G, F>
where
    G: Gen<T>,
    F: Fn(T) -> U,
{
    fn gen(&self, fate: &mut Fate) -> U {
        let g = &self.g;
        let f = &self.f;

        let t = g.gen(fate);
        f(t)
    }
}
