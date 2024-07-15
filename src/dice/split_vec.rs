use crate::prelude::*;

/// Generates `N` [`Vec`]s by splitting the given [`Vec`] at random indices.
///
/// # Panics
///
/// Panics if `!vec.is_empty() && N == 0` is true.
///
/// # Examples
///
/// This example splits [`Vec`]s without panicking:
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
/// let [prefix, suffix] = fate.roll(dice::split_vec(vec.clone()));
/// assert!(vec.starts_with(&prefix));
/// assert!(vec.ends_with(&suffix));
///
/// let empty_vec: Vec<u64> = vec![];
/// let [empty_prefix, empty_suffix] = fate.roll(dice::split_vec(empty_vec));
/// assert!(empty_prefix.is_empty());
/// assert!(empty_suffix.is_empty());
/// ```
///
/// This example panics:
///
/// ```should_panic
/// use dicetest::prelude::*;
///
/// let vec = vec![1, 2, 3, 4];
/// // Oh no, panic!
/// let _parts_die = dice::split_vec::<_, 0>(vec);
/// ```
#[track_caller]
pub fn split_vec<T, const N: usize>(mut vec: Vec<T>) -> impl DieOnce<[Vec<T>; N]> {
    let sizes_die = dice::split_usize(vec.len());
    dice::from_fn_once(move |mut fate| {
        let sizes = fate.roll(sizes_die);
        let mut parts = sizes.map(|size| vec.split_off(vec.len() - size));
        parts.reverse();
        parts
    })
}

/// Generates `n` [`Vec`]s by splitting the given [`Vec`] at random indices.
///
/// # Panics
///
/// Panics if `!vec.is_empty() && n == 0` is true.
///
/// # Examples
///
/// This example splits [`Vec`]s without panicking:
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
/// let parts = fate.roll(dice::split_vec_n(vec.clone(), 2));
/// assert!(vec.starts_with(&parts[0]));
/// assert!(vec.ends_with(&parts[1]));
///
/// let empty_vec: Vec<u64> = vec![];
/// let parts = fate.roll(dice::split_vec_n(empty_vec, 2));
/// assert!(parts[0].is_empty());
/// assert!(parts[1].is_empty());
/// ```
///
/// This example panics:
///
/// ```should_panic
/// use dicetest::prelude::*;
///
/// let vec = vec![1, 2, 3, 4];
/// // Oh no, panic!
/// let _parts_die = dice::split_vec_n(vec, 0);
/// ```
#[track_caller]
pub fn split_vec_n<T>(mut vec: Vec<T>, n: usize) -> impl DieOnce<Vec<Vec<T>>> {
    let sizes_die = dice::split_usize_n(vec.len(), n);
    dice::from_fn_once(move |mut fate| {
        let sizes = fate.roll(sizes_die);
        let mut parts = sizes
            .into_iter()
            .map(|size| vec.split_off(vec.len() - size))
            .collect::<Vec<_>>();
        parts.reverse();
        parts
    })
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    fn merge_parts(parts: &[Vec<u8>]) -> Vec<u8> {
        let mut merged = Vec::new();
        for part in parts {
            merged.extend(part);
        }
        merged
    }

    #[test]
    fn split_vec_result_can_be_merged_to_orig_vec() {
        Dicetest::repeatedly().run(|mut fate| {
            let orig_vec = fate.roll(dice::vec(dice::u8(..), ..));

            let parts = fate.roll(dice::split_vec::<_, 1>(orig_vec.clone()));
            let merged_vec = merge_parts(&parts);
            assert_eq!(merged_vec, orig_vec);

            let parts = fate.roll(dice::split_vec::<_, 2>(orig_vec.clone()));
            let merged_vec = merge_parts(&parts);
            assert_eq!(merged_vec, orig_vec);

            let parts = fate.roll(dice::split_vec::<_, 3>(orig_vec.clone()));
            let merged_vec = merge_parts(&parts);
            assert_eq!(merged_vec, orig_vec);

            let parts = fate.roll(dice::split_vec::<_, 4>(orig_vec.clone()));
            let merged_vec = merge_parts(&parts);
            assert_eq!(merged_vec, orig_vec);
        })
    }

    #[test]
    fn split_vec_n_result_can_be_merged_to_orig_vec() {
        Dicetest::repeatedly().run(|mut fate| {
            let expected_count = fate.roll(dice::length(..));
            let orig_vec = if expected_count == 0 {
                Vec::new()
            } else {
                fate.roll(dice::vec(dice::u8(..), ..))
            };
            let parts = fate.roll(dice::split_vec_n(orig_vec.clone(), expected_count));

            let actual_count = parts.len();
            let merged_vec = merge_parts(&parts);

            assert_eq!(actual_count, expected_count);
            assert_eq!(merged_vec, orig_vec);
        })
    }

    #[test]
    fn split_vec_with_zero() {
        Dicetest::repeatedly().run(|mut fate| {
            let parts = fate.roll(dice::split_vec::<u8, 0>(Vec::new()));
            assert_eq!(parts, [] as [Vec<u8>; 0]);
        })
    }

    #[test]
    fn split_vec_n_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                stat!(
                    "split_vec_n(vec![1, 2, 3, 4, 5], 2)",
                    "{:?}",
                    fate.roll(dice::split_vec_n(vec![1, 2, 3, 4, 5], 2)),
                );

                stat!(
                    "split_vec_n(vec![1, 2, 3, 4, 5], 3)",
                    "{:?}",
                    fate.roll(dice::split_vec_n(vec![1, 2, 3, 4, 5], 3)),
                );
            })
    }
}
