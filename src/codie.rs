/// The counterpart of `DieOnce` and `Die`. Instead of generating a value of type `T` from a seed,
/// this trait allows to convert a value of type `T` into a seed.
pub trait Codie<T> {
    /// Converts the given value into a seed. The implementation must be deterministic.
    fn coroll(&self, value: T) -> u64;
}
