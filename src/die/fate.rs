use crate::die::{Limit, Prng};

/// Represents the randomness used by `DieOnce::roll_once` and `Die::roll`.
pub struct Fate<'a> {
    pub prng: &'a mut Prng,
    limit: Limit,
}

impl<'a> Fate<'a> {
    pub fn new(prng: &'a mut Prng, limit: Limit) -> Self {
        Fate { prng, limit }
    }

    pub fn limit(&self) -> Limit {
        self.limit
    }
}
