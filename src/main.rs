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

fn main() {
    let mut v: Vec<u8> = vec![0, 2, 2, 4, 1, 0, 2, 5];
    println!("unsorted: {:?}", v);
    bubble_sort(&mut v);
    println!("  sorted: {:?}", v);
}

#[cfg(test)]
mod tests {
    use dicetest::prelude::tests::*;

    use super::*;

    #[ignore]
    #[test]
    fn result_of_bubble_sort_is_sorted() {
        dicetest!(|fate| {
            let mut v = dice::vec(dice::u8(..), ..).roll(fate);
            hint!("unsorted: {:?}", v);

            bubble_sort(&mut v);
            hint!("  sorted: {:?}", v);

            let is_sorted = v.windows(2).all(|w| w[0] <= w[1]);
            assert!(is_sorted);
        })
    }
}
