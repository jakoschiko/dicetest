use crate::prelude::dice::*;

/// Shuffles the given `Vec` randomly using the [Fisher-Yates shuffle].
///
/// [Fisher-Yates shuffle]: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
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
            hint!(orig_vec);

            let shuffled_vec = dice::shuffled_vec(orig_vec).roll_once(fate);
            let shuffled_vec_elems = count_vec_elems(&shuffled_vec);
            hint!(shuffled_vec);

            assert_eq!(orig_vec_elems, shuffled_vec_elems);
        })
    }
}
