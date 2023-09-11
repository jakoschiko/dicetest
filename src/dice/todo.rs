use crate::prelude::*;

/// Generator for prototyping that always panics when generating a value.
///
/// The `todo!()` macro doesn't work in places where a `impl Die` or `impl DieOnce` is expected
/// because the type `!` is not stabilized yet, hence it's not possible to implement `Die` or
/// `DieOnce` for `!`.
///
/// # Examples
///
/// This example doesn't panic because the generator isn't used:
///
/// ```
/// use dicetest::prelude::*;
///
/// let _byte_die = dice::todo::<u8>();
/// ```
///
/// However, this example panics:
///
/// ```should_panic
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let _byte = fate.roll(dice::todo::<u32>());
/// ```
pub fn todo<T>() -> impl Die<T> {
    dice::from_fn(|_| {
        panic!(
            "implementation for Die<{}> is missing",
            std::any::type_name::<T>(),
        )
    })
}
