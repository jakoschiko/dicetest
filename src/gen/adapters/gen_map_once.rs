use std::marker::PhantomData;

use ::rng::Rng;
use ::gen::{Size, GenOnce};

/// Adapter for `GenOnce::map_once`.
pub struct GenMapOnce<T, U, G, F>
where
    G: GenOnce<T>,
    F: FnOnce(T) -> U,
{
    g: G,
    f: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

impl<T, U, G, F> GenMapOnce<T, U, G, F>
where
    G: GenOnce<T>,
    F: FnOnce(T) -> U,
{
    pub fn new(g: G, f: F) -> Self {
        GenMapOnce {
            g,
            f,
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<T, U, G, F> GenOnce<U> for GenMapOnce<T, U, G, F>
where
    G: GenOnce<T>,
    F: FnOnce(T) -> U,
{
    fn gen_once(self, rng: &mut Rng, size: Size) -> U {
        let g = self.g;
        let f = self.f;

        let t = g.gen_once(rng, size);
        let u = f(t);

        u
    }
}
