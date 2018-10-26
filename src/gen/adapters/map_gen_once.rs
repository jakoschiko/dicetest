use std::marker::PhantomData;

use crate::rng::Rng;
use crate::gen::{Limit, GenOnce};

/// Adapter for `GenOnce::map_once`.
pub struct MapGenOnce<T, U, G, F>
where
    G: GenOnce<T>,
    F: FnOnce(T) -> U,
{
    g: G,
    f: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

impl<T, U, G, F> MapGenOnce<T, U, G, F>
where
    G: GenOnce<T>,
    F: FnOnce(T) -> U,
{
    pub fn new(g: G, f: F) -> Self {
        MapGenOnce {
            g,
            f,
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<T, U, G, F> GenOnce<U> for MapGenOnce<T, U, G, F>
where
    G: GenOnce<T>,
    F: FnOnce(T) -> U,
{
    fn gen_once(self, rng: &mut Rng, lim: Limit) -> U {
        let g = self.g;
        let f = self.f;

        let t = g.gen_once(rng, lim);
        let u = f(t);

        u
    }
}
