use crate::rng::Rng;
use crate::gen::Limit;

/// Represents the randomness used by `GenOnce::gen` and `Gen::gen`.
pub struct Dice<'a> {
    pub rng: &'a mut Rng,
    limit: Limit,
}

impl<'a> Dice<'a> {
    pub fn new(rng: &'a mut Rng, limit: Limit) -> Self {
        Dice { rng, limit }
    }

    pub fn limit(&self) -> Limit {
        self.limit
    }
}
