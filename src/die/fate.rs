use crate::die::DieOnce;
use crate::die::Limit;
use crate::prand::Prng;

/// Contains all parameters that are needed to use `DieOnce` and `Die`.
///
/// This struct exists mainly for convenience reasons.
pub struct Fate<'a> {
    pub prng: &'a mut Prng,
    pub limit: Limit,
}

impl<'a> Fate<'a> {
    /// Uses the underlying parameters to generate a random value with the given `DieOnce`.
    pub fn roll<T>(&mut self, die: impl DieOnce<T>) -> T {
        die.roll_once(self.prng, self.limit)
    }
}
