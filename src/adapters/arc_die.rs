use std::sync::Arc;

use crate::{Die, DieOnce, Fate};

/// Adapter for `Die::arc`.
#[derive(Clone)]
pub struct ArcDie<T> {
    die: Arc<dyn Die<T>>,
}

impl<T> ArcDie<T> {
    pub fn new<D>(die: D) -> Self
    where
        D: Die<T> + 'static,
    {
        let die = Arc::new(die);
        ArcDie { die }
    }
}

impl<T> Die<T> for ArcDie<T> {
    fn roll(&self, fate: &mut Fate) -> T {
        self.die.roll(fate)
    }
}

impl<T> DieOnce<T> for ArcDie<T> {
    fn roll_once(self, fate: &mut Fate) -> T {
        self.roll(fate)
    }
}
