//! The standard collection of `Cogens` implementations.

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::prelude::cogens::*;

/// Uses stdlib's `DefaultHasher` to create a seed from a hashable value.
pub fn from_default_hasher<T: Hash>() -> impl Cogen<T> {
    |value: T| {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::cogen::Cogen;
    use crate::cogens;

    #[test]
    fn from_default_hasher_is_deterministic() {
        let cogen_0 = cogens::from_default_hasher::<u8>();
        let cogen_1 = cogens::from_default_hasher::<u8>();

        assert_eq!(cogen_0.cogen(42), cogen_1.cogen(42));
    }
}