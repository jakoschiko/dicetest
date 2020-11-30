use std::marker::PhantomData;

use crate::{Die, DieOnce, Fate};

/// Adapter for [`DieOnce::map_once`] and [`Die::map`].
pub struct MapDie<T, U, D, F> {
    d: D,
    f: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

impl<T, U, D, F> MapDie<T, U, D, F> {
    pub fn new(d: D, f: F) -> Self {
        MapDie {
            d,
            f,
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<T, U, D, F> DieOnce<U> for MapDie<T, U, D, F>
where
    D: DieOnce<T>,
    F: FnOnce(T) -> U,
{
    fn roll_once(self, fate: Fate) -> U {
        let d = self.d;
        let f = self.f;

        let t = d.roll_once(fate);
        f(t)
    }
}

impl<T, U, D, F> Die<U> for MapDie<T, U, D, F>
where
    D: Die<T>,
    F: Fn(T) -> U,
{
    fn roll(&self, fate: Fate) -> U {
        let d = &self.d;
        let f = &self.f;

        let t = d.roll(fate);
        f(t)
    }
}
