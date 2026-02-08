use crate::Seed;

/// Trait for converting values of type `T` into a seed.
///
/// This trait can be seen as the counterpart of [`DieOnce`] and [`Die`].
/// Instead of generating a value of type `T` from a seed, this trait allows converting
/// a value of type `T` into a seed.
///
/// The most important use case is implementing pseudorandom functions. A pseudorandom function
/// can be implemented by converting its arguments into a seed and then using the seed for
/// generating a result.
///
/// [`DieOnce`]: crate::DieOnce
/// [`Die`]: crate::Die
pub trait Codie<T> {
    /// Converts the given value into a seed. The implementation must be deterministic.
    fn coroll(&self, value: T) -> Seed;
}
