use crate::gen::{Limit, Prng};

/// Represents the randomness used by `GenOnce::gen` and `Gen::gen`.
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
