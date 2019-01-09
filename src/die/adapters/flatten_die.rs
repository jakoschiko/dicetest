use std::marker::PhantomData;

use crate::die::{Fate, Die, DieOnce};

/// Adapter for `DieOnce::flatten_once` and `Die::flatten`.
pub struct FlattenDie<T, TD, TDD> {
    td_die: TDD,
    _t: PhantomData<T>,
    _t_die: PhantomData<TD>,
}

impl<T, TD, TDD> FlattenDie<T, TD, TDD> {
    pub fn new(td_die: TDD) -> Self {
        FlattenDie {
            td_die,
            _t: PhantomData,
            _t_die: PhantomData,
        }
    }
}

impl<T, TD, TDD> DieOnce<T> for FlattenDie<T, TD, TDD>
where
    TD: DieOnce<T>,
    TDD: DieOnce<TD>,
{
    fn roll_once(self, fate: &mut Fate) -> T {
        let td_die = self.td_die;

        let t_die = td_die.roll_once(fate);
        t_die.roll_once(fate)
    }
}

impl<T, TD, TDD> Die<T> for FlattenDie<T, TD, TDD>
where
    TD: DieOnce<T>,
    TDD: Die<TD>,
{
    fn roll(&self, fate: &mut Fate) -> T {
        let td_die = &self.td_die;

        let t_die = td_die.roll(fate);
        t_die.roll_once(fate)
    }
}
