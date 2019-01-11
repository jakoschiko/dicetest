use crate::die::Limit;
use crate::prand::Prng;

/// Contains all parameters needed to use `DieOnce` and `Die`.
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
