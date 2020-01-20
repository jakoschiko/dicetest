#![allow(dead_code)]

fn bubble_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();

    for _ in 0..len {
        for j in 1..len - 1 {
            let jpp = j + 1;
            if slice[j] > slice[jpp] {
                slice.swap(j, jpp);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicetest::prelude::tests::*;

    #[test]
    fn result_of_bubble_sort_is_sorted() {
        dicetest!(|mut fate| {
            let mut v = fate.roll(dice::vec(dice::u8(..), ..));
            hint!("unsorted: {:?}", v);

            bubble_sort(&mut v);
            hint!("  sorted: {:?}", v);

            let is_sorted = v.windows(2).all(|w| w[0] <= w[1]);
            assert!(is_sorted);
        })
    }
}
