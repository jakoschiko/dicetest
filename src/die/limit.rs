use std::convert::TryFrom;

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
    /// Uses the given `usize` as limit.
    ///
    /// If the `usize` is greater than the largest `u64` value,
    /// the function returns the largest `u64` value as limit.
    pub fn saturating_from_usize(usize: usize) -> Self {
        Self(u64::try_from(usize).unwrap_or(u64::max_value()))
    }

    /// Returns the limit as `usize`.
    ///
    /// If the limit is greater than the largest `usize` value,
    /// the function returns the largest `usize` value.
    pub fn saturating_to_usize(self) -> usize {
        usize::try_from(self.0).unwrap_or(usize::max_value())
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
