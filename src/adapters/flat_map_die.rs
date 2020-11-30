use std::marker::PhantomData;

use crate::{Die, DieOnce, Fate};

/// Adapter for [`DieOnce::flat_map_once`] and [`Die::flat_map`].
pub struct FlatMapDie<T, U, TD, UD, F> {
    t_die: TD,
    f: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
    _u_die: PhantomData<UD>,
}

impl<T, U, TD, UD, F> FlatMapDie<T, U, TD, UD, F> {
    pub fn new(t_die: TD, f: F) -> Self {
        FlatMapDie {
            t_die,
            f,
            _t: PhantomData,
            _u: PhantomData,
            _u_die: PhantomData,
        }
    }
}

impl<T, U, TD, UD, F> DieOnce<U> for FlatMapDie<T, U, TD, UD, F>
where
    TD: DieOnce<T>,
    UD: DieOnce<U>,
    F: FnOnce(T) -> UD,
{
    fn roll_once(self, mut fate: Fate) -> U {
        let t_die = self.t_die;
        let f = self.f;

        let t = t_die.roll_once(fate.copy());
        let gu = f(t);
        gu.roll_once(fate)
    }
}

impl<T, U, TD, UD, F> Die<U> for FlatMapDie<T, U, TD, UD, F>
where
    TD: Die<T>,
    UD: DieOnce<U>,
    F: Fn(T) -> UD,
{
    fn roll(&self, mut fate: Fate) -> U {
        let t_die = &self.t_die;
        let f = &self.f;

        let t = t_die.roll(fate.copy());
        let gu = f(t);
        gu.roll_once(fate)
    }
}
