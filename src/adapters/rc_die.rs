use std::rc::Rc;

use crate::{Die, DieOnce, Fate};

/// Adapter for [`Die::rc`].
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
    fn roll(&self, fate: Fate) -> T {
        self.die.roll(fate)
    }
}

impl<'a, T> DieOnce<T> for RcDie<'a, T> {
    fn roll_once(self, fate: Fate) -> T {
        self.roll(fate)
    }
}
