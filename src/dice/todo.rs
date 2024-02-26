use crate::prelude::*;

/// Generator for prototyping that panics when created.
///
/// The `todo!()` macro doesn't work in places where a `impl Die` or `impl DieOnce` is expected
/// because the type `!` is not stabilized yet, hence it's not possible to implement `Die` or
/// `DieOnce` for `!`. This function can be used as an alternative.
///
/// # Examples
///
/// This example panics:
///
/// ```should_panic
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let _number_die = dice::todo::<u32>();
/// ```
pub fn todo<T>() -> impl Die<T> {
    panic!(
        "implementation for Die<{}> is missing",
        std::any::type_name::<T>(),
    );
    #[allow(unreachable_code)]
    dice::from_fn(|_| unreachable!())
}
