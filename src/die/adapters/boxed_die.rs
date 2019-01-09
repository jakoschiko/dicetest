use crate::die::{Fate, Die, DieOnce};

/// Adapter for `Die::boxed`.
pub struct BoxedDie<'a, T> {
    die: Box<dyn Die<T> + 'a>,
}

impl<'a, T> BoxedDie<'a, T> {
    pub fn new<D>(die: D) -> Self
    where
        D: Die<T> + 'a,
    {
        let die = Box::new(die);
        BoxedDie { die }
    }
}

impl<'a, T> Die<T> for BoxedDie<'a, T> {
    fn roll(&self, fate: &mut Fate) -> T {
        self.die.roll(fate)
    }
}

impl<'a, T> DieOnce<T> for BoxedDie<'a, T> {
    fn roll_once(self, fate: &mut Fate) -> T {
        self.roll(fate)
    }
}
