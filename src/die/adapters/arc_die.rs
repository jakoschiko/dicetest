use std::sync::Arc;

use crate::die::{Die, DieOnce, Limit};
use crate::prand::Prng;

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
    fn roll(&self, prng: &mut Prng, limit: Limit) -> T {
        self.die.roll(prng, limit)
    }
}

impl<T> DieOnce<T> for ArcDie<T> {
    fn roll_once(self, prng: &mut Prng, limit: Limit) -> T {
        self.roll(prng, limit)
    }
}
