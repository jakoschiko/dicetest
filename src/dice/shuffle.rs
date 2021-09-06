use crate::prelude::*;

/// Shuffles the given slice randomly in-place using the [Fisher-Yates shuffle].
///
/// [Fisher-Yates shuffle]: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
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
/// let mut elems = vec![1, 2, 3, 4];
///
/// fate.roll(dice::shuffle_slice(&mut elems));
/// ```
pub fn shuffle_slice<T>(elems: &'_ mut [T]) -> impl DieOnce<()> + '_ {
    dice::from_fn_once(move |mut fate| {
        let n = elems.len();
        if n > 0 {
            for i in 0..(n - 1) {
                let j = fate.roll(dice::uni_usize(i..n));
                elems.swap(i, j);
            }
        }
    })
}

/// Shuffles the given [`Vec`] randomly using the [Fisher-Yates shuffle].
///
/// [Fisher-Yates shuffle]: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
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
/// let sorted = vec![1, 2, 3, 4];
///
/// let probably_unsorted = fate.roll(dice::shuffled_vec(sorted));
/// ```
pub fn shuffled_vec<T>(mut vec: Vec<T>) -> impl DieOnce<Vec<T>> {
    dice::from_fn_once(move |mut fate| {
        fate.roll(shuffle_slice(&mut vec));
        vec
    })
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashMap;

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
        Dicetest::repeatedly().run(|mut fate| {
            let orig_vec = fate.roll(dice::vec(dice::u8(..), ..));
            let orig_vec_elems = count_vec_elems(&orig_vec);
            hint_debug!(orig_vec);

            let shuffled_vec = fate.roll(dice::shuffled_vec(orig_vec));
            let shuffled_vec_elems = count_vec_elems(&shuffled_vec);
            hint_debug!(shuffled_vec);

            assert_eq!(orig_vec_elems, shuffled_vec_elems);
        })
    }

    #[test]
    fn shuffled_vec_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                stat!(
                    "shuffled_vec(vec![1, 2, 3])",
                    "{:?}",
                    fate.roll(dice::shuffled_vec(vec![1, 2, 3])),
                );
            })
    }
}
