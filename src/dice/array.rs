use crate::prelude::*;

/// Generates an array with random elements.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let byte_die = dice::u8(..);
/// let bytes_die = dice::array(byte_die);
/// let [a, b, c, d]  = fate.roll(bytes_die);
/// ```
pub fn array<T, D: Die<T>, const N: usize>(elem_die: D) -> impl Die<[T; N]> {
    dice::from_fn(move |mut fate| [(); N].map(|_| fate.roll(&elem_die)))
}
