use crate::prelude::gens::*;

/// Shuffles the given `Vec` randomly using the [Fisher-Yates shuffle].
///
/// [Fisher-Yates shuffle]: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
pub fn shuffled_vec<T>(mut vec: Vec<T>) -> impl GenOnce<Vec<T>> {
    gens::from_fn_once(move |dice| {
        let n = vec.len();
        if n > 0 {
            for i in 0..(n - 1) {
                let j = gens::usize_uniform(i..n).gen(dice);
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
        dicetest!(|dice| {
            let orig_vec = gens::vec(gens::u8(..), ..).gen(dice);
            let orig_vec_elems = count_vec_elems(&orig_vec);
            hint_dbg!(&orig_vec);

            let shuffled_vec = gens::shuffled_vec(orig_vec).gen_once(dice);
            let shuffled_vec_elems = count_vec_elems(&shuffled_vec);
            hint_dbg!(&shuffled_vec);

            assert_eq!(orig_vec_elems, shuffled_vec_elems);
        })
    }
}
