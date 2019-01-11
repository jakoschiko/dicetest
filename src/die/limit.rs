use std::mem::size_of;

/// Recommendation for the upper size limit of dynamic data structures generated with `DieOnce`
/// and `Die`.
///
/// The generator implementation is allowed to freely interpret this value, but the complexity of
/// the value generation should be in `O(limit)`.
///
/// This parameter exists because the hardware of the testing machine is limited. For example
/// a very big list could not fit in the memory or its generation could take too much time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Limit(pub u64);

impl Limit {
    /// Returns the limit as `usize`. If the limit is greater than the largest `usize` value,
    /// the function returns the largest `usize` value.
    pub fn saturating_to_usize(self) -> usize {
        let limit_fits_into_usize = {
            let usize_is_big_enough = size_of::<u64>() <= size_of::<usize>();
            let limit_is_small_enough = || self.0 >= usize::max_value() as u64;
            usize_is_big_enough || limit_is_small_enough()
        };

        if limit_fits_into_usize {
            self.0 as usize
        } else {
            usize::max_value()
        }
    }
}

impl From<u64> for Limit {
    fn from(limit: u64) -> Self {
        Limit(limit)
    }
}

impl Default for Limit {
    fn default() -> Self {
        Limit(100)
    }
}
