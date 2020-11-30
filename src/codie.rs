//! Provides the trait [`Codie`] for converting values into a seed.

use crate::Seed;

/// The counterpart of [`DieOnce`] and [`Die`]. Instead of generating a value of type
/// `T` from a seed, this trait allows to convert a value of type `T` into a seed.
///
/// [`DieOnce`]: crate::DieOnce
/// [`Die`]: crate::Die
pub trait Codie<T> {
    /// Converts the given value into a seed. The implementation must be deterministic.
    fn coroll(&self, value: T) -> Seed;
}
