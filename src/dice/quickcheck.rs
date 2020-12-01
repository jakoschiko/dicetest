use quickcheck::Arbitrary;

use crate::prelude::*;

/// Generates a value using the [`Arbitrary`] implementation of the type `T`.
///
/// Only available if the feature `quickcheck_full` is enabled.
pub fn arbitrary<T>() -> impl Die<T>
where
    T: Arbitrary,
{
    dice::from_fn(|mut fate| T::arbitrary(&mut fate))
}
