use crate::prelude::dice::*;

/// Generates two `Vecs` by splitting the given `Vec` at a random index.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::dice::*;
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
///
/// Fate::run(&mut prng, limit, |fate| {
///     let vec = vec![1, 2, 3, 4];
///     let (prefix, suffix) = dice::split_vec(vec.clone()).roll_once(fate);
///     assert!(vec.starts_with(&prefix));
///     assert!(vec.ends_with(&suffix));
///
///     let empty_vec: Vec<u64> = vec![];
///     let (empty_prefix, empty_suffix) = dice::split_vec(empty_vec).roll_once(fate);
///     assert!(empty_prefix.is_empty());
///     assert!(empty_suffix.is_empty());
/// });
/// ```
pub fn split_vec<T>(mut vec: Vec<T>) -> impl DieOnce<(Vec<T>, Vec<T>)> {
    let index_die = dice::uni_usize(0..=vec.len());
    dice::from_fn_once(move |fate| {
        let at = index_die.roll(fate);
        let other_vec = vec.split_off(at);
        (vec, other_vec)
    })
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;

    #[test]
    fn split_vec_result_can_be_merged_to_orig_vec() {
        dicetest!(|fate| {
            let orig_vec = dice::vec(dice::u8(..), ..).roll(fate);
            let (prefix, mut suffix) = dice::split_vec(orig_vec.clone()).roll_once(fate);

            let mut merged = prefix;
            merged.append(&mut suffix);

            assert_eq!(merged, orig_vec);
        })
    }

    #[test]
    fn split_vec_calc_stats() {
        dicetest!(Config::default().with_passes(0), |fate| {
            stat!(
                "split_vec(vec![1, 2, 3, 4, 5])",
                "{:?}",
                dice::split_vec(vec![1, 2, 3, 4, 5]).roll_once(fate),
            );
        })
    }
}
