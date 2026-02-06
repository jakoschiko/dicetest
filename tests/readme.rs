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
    use dicetest::prelude::*;

    #[test]
    #[should_panic] // Should fail in readme
    fn result_of_bubble_sort_is_sorted() {
        Dicetest::repeatedly().run(|mut fate| {
            let mut vec: Vec<u8> = fate.roll(die());
            hint!("unsorted: {:?}", vec);

            bubble_sort(&mut vec);
            hint!("  sorted: {:?}", vec);

            let is_sorted = vec.windows(2).all(|w| w[0] <= w[1]);
            assert!(is_sorted);
        })
    }
}
