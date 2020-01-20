use std::rc::Rc;

use crate::die::{Die, DieOnce, Limit};
use crate::prand::Prng;

/// Adapter for `DieOnce::rc`.
#[derive(Clone)]
pub struct RcDie<'a, T> {
    die: Rc<dyn Die<T> + 'a>,
}

impl<'a, T> RcDie<'a, T> {
    pub fn new<D>(die: D) -> Self
    where
        D: Die<T> + 'a,
    {
        let die = Rc::new(die);
        RcDie { die }
    }
}

impl<'a, T> Die<T> for RcDie<'a, T> {
    fn roll(&self, prng: &mut Prng, limit: Limit) -> T {
        self.die.roll(prng, limit)
    }
}

impl<'a, T> DieOnce<T> for RcDie<'a, T> {
    fn roll_once(self, prng: &mut Prng, limit: Limit) -> T {
        self.roll(prng, limit)
    }
}
