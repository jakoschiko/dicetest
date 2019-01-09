use std::rc::Rc;

use crate::die::{Fate, Die, DieOnce};

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
    fn roll(&self, fate: &mut Fate) -> T {
        self.die.roll(fate)
    }
}

impl<'a, T> DieOnce<T> for RcDie<'a, T> {
    fn roll_once(self, fate: &mut Fate) -> T {
        self.roll(fate)
    }
}
