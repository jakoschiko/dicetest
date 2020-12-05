//! The standard collection of [`Codie`] implementations.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::prelude::*;
use crate::Seed;

struct Fun<F>(F);

impl<T, F> Codie<T> for Fun<F>
where
    F: Fn(T) -> Seed,
{
    fn coroll(&self, value: T) -> Seed {
        self.0(value)
    }
}

/// Helper for implementing a [`Codie`] from a [`Fn`] that returns a seed.
pub fn from_fn<T, F>(f: F) -> impl Codie<T>
where
    F: Fn(T) -> Seed,
{
    Fun(f)
}

/// Uses libstd's [`DefaultHasher`] to create a seed from a hashable value.
pub fn from_default_hasher<T: Hash>() -> impl Codie<T> {
    from_fn(|value: T| {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish().into()
    })
}

#[cfg(test)]
mod tests {
    use crate::codice;
    use crate::codice::Codie;

    #[test]
    fn from_default_hasher_is_deterministic() {
        let codie_0 = codice::from_default_hasher::<u8>();
        let codie_1 = codice::from_default_hasher::<u8>();

        assert_eq!(codie_0.coroll(42), codie_1.coroll(42));
    }
}
