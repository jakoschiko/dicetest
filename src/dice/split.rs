use crate::prelude::*;

/// Generates two `Vec`s by splitting the given `Vec` at a random index.
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
/// let vec = vec![1, 2, 3, 4];
/// let (prefix, suffix) = fate.roll(dice::split_vec(vec.clone()));
/// assert!(vec.starts_with(&prefix));
/// assert!(vec.ends_with(&suffix));
///
/// let empty_vec: Vec<u64> = vec![];
/// let (empty_prefix, empty_suffix) = fate.roll(dice::split_vec(empty_vec));
/// assert!(empty_prefix.is_empty());
/// assert!(empty_suffix.is_empty());
/// ```
pub fn split_vec<T>(mut vec: Vec<T>) -> impl DieOnce<(Vec<T>, Vec<T>)> {
    let index_die = dice::uni_usize(0..=vec.len());
    dice::from_fn_once(move |mut fate| {
        let at = fate.roll(index_die);
        let other_vec = vec.split_off(at);
        (vec, other_vec)
    })
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn split_vec_result_can_be_merged_to_orig_vec() {
        Dicetest::repeatedly().run(|mut fate| {
            let orig_vec = fate.roll(dice::vec(dice::u8(..), ..));
            let (prefix, mut suffix) = fate.roll(dice::split_vec(orig_vec.clone()));

            let mut merged = prefix;
            merged.append(&mut suffix);

            assert_eq!(merged, orig_vec);
        })
    }

    #[test]
    fn split_vec_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                stat!(
                    "split_vec(vec![1, 2, 3, 4, 5])",
                    "{:?}",
                    fate.roll(dice::split_vec(vec![1, 2, 3, 4, 5])),
                );
            })
    }
}
