use crate::gen::{Limit, Prng};

/// Represents the randomness used by `GenOnce::gen` and `Gen::gen`.
pub struct Dice<'a> {
    pub prng: &'a mut Prng,
    limit: Limit,
}

impl<'a> Dice<'a> {
    pub fn new(prng: &'a mut Prng, limit: Limit) -> Self {
        Dice { prng, limit }
    }

    pub fn limit(&self) -> Limit {
        self.limit
    }
}
