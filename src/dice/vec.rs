use crate::dice::{CollectionBuilder, SizeRange};
use crate::prelude::dice::*;

/// [`Vec`] builder for [`dice::collection`].
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`dice::collection`]: fn.collection.html
#[derive(Clone)]
pub struct VecBuilder;

impl<T> CollectionBuilder<T, Vec<T>> for VecBuilder {
    fn build(self, elems: impl ExactSizeIterator<Item = T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(elems.len());
        vec.extend(elems);
        vec
    }
}

/// Generates a [`Vec`] that contains elements of type `T`.
///
/// The range specifies the length of the [`Vec`].
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
///
/// # Panics
///
/// Panics if the range is empty.
pub fn vec<T>(elem_die: impl Die<T>, len_range: impl SizeRange) -> impl Die<Vec<T>> {
    let builder_die = dice::just(VecBuilder);
    dice::collection(builder_die, elem_die, len_range)
}

/// Shuffles the given `Vec` randomly using the [Fisher-Yates shuffle].
///
/// [Fisher-Yates shuffle]: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let sorted = vec![1, 2, 3, 4];
///
/// let probably_unsorted = dice::shuffled_vec(sorted).sample_once();
/// ```
pub fn shuffled_vec<T>(mut vec: Vec<T>) -> impl DieOnce<Vec<T>> {
    dice::from_fn_once(move |fate| {
        let n = vec.len();
        if n > 0 {
            for i in 0..(n - 1) {
                let j = dice::uni_usize(i..n).roll(fate);
                vec.swap(i, j);
            }
        }
        vec
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::prelude::tests::*;

    fn count_vec_elems(vec: &Vec<u8>) -> HashMap<u8, usize> {
        let mut elems = HashMap::new();
        for &elem in vec.iter() {
            let count = elems.entry(elem).or_insert(0);
            *count += 1;
        }
        elems
    }

    #[test]
    fn shuffled_vec_contains_same_elems() {
        dicetest!(|fate| {
            let orig_vec = dice::vec(dice::u8(..), ..).roll(fate);
            let orig_vec_elems = count_vec_elems(&orig_vec);
            hint_debug!(orig_vec);

            let shuffled_vec = dice::shuffled_vec(orig_vec).roll_once(fate);
            let shuffled_vec_elems = count_vec_elems(&shuffled_vec);
            hint_debug!(shuffled_vec);

            assert_eq!(orig_vec_elems, shuffled_vec_elems);
        })
    }
}
