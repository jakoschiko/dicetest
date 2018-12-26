use std::marker::PhantomData;

use crate::gen::{Dice, GenOnce, Gen};

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
    fn gen_once(self, dice: &mut Dice) -> U {
        let g = self.g;
        let f = self.f;

        let t = g.gen_once(dice);
        f(t)
    }
}

impl<T, U, G, F> Gen<U> for MapGen<T, U, G, F>
where
    G: Gen<T>,
    F: Fn(T) -> U,
{
    fn gen(&self, dice: &mut Dice) -> U {
        let g = &self.g;
        let f = &self.f;

        let t = g.gen(dice);
        f(t)
    }
}
