/// The counterpart of `GenOnce` and `Gen`. Instead of generating a value of type `T` from a seed,
/// this trait allows to convert a value of type `T` into a seed.
pub trait Cogen<T> {
    /// Converts the given value into a seed. The implementation must be deterministic.
    fn cogen(&self, value: T) -> u64;
}
