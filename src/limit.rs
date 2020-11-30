use std::convert::TryFrom;

/// The upper limit for the size of dynamic data structures generated with [`DieOnce`] and [`Die`].
///
/// The implementor of [`DieOnce`] or [`Die`] is allowed to freely interpret or even ignore this value,
/// but it's recommended that the complexity of the value generation is in `O(limit)`.
///
/// This parameter exists because the hardware of the testing machine is limited. For example
/// a very big list could not fit in the memory or its generation could take too much time.
/// With this parameter you can implement a generator for lists of arbitrary size and its
/// user can choose an upper limit depending on his hardware.
///
/// [`DieOnce`]: crate::DieOnce
/// [`Die`]: crate::Die
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Limit(pub u64);

impl Limit {
    /// Uses the given [`usize`] as limit.
    ///
    /// If the [`usize`] is greater than the largest [`u64`] value,
    /// the function returns the largest [`u64`] value as limit.
    pub fn saturating_from_usize(usize: usize) -> Self {
        Self(u64::try_from(usize).unwrap_or(u64::max_value()))
    }

    /// Returns the limit as `usize`.
    ///
    /// If the limit is greater than the largest [`usize`] value,
    /// the function returns the largest [`usize`] value.
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
