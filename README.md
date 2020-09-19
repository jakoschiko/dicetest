# Dicetest

Dicetest is a framework for writing tests with randomly generated test data.

## Status of this crate

The author does not consider this crate as stable yet.

## Example

Here's an example of an incorrect sort function tested with Dicetest:
```rust
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
        Dicetest::repeatedly().run(|fate| {
            let mut v = dice::vec(dice::u8(..), ..).roll(fate);
            hint!("unsorted: {:?}", v);

            bubble_sort(&mut v);
            hint!("  sorted: {:?}", v);

            let is_sorted = v.windows(2).all(|w| w[0] <= w[1]);
            assert!(is_sorted);
        })
    }
}
```

Running `cargo test` produces the following output:
```text
The test failed after 31 passes.

# Config
- seed: 3713861809241954222
- start limit: 0
- end limit: 100
- passes: 1000

# Counterexample
- run code: "/yiA1sab3S4UnCf4ozyMpxMxzg1NtFybCuYLHy0/oscDAAAAAAAAAA=="
- limit: 3
- hints:
    - unsorted: [201, 209, 2]
    -   sorted: [201, 2, 209]
- error: assertion failed: is_sorted
```

You can rerun the counterexample by setting an environment variable:
```text
DICETEST_DEBUG=/yiA1sab3S4UnCf4ozyMpxMxzg1NtFybCuYLHy0/oscDAAAAAAAAAA== cargo test
```

Or you can modify the test:
```rust
Dicetest::debug("/yiA1sab3S4UnCf4ozyMpxMxzg1NtFybCuYLHy0/oscDAAAAAAAAAA==").run(|fate| {
    // ...
})
```

For more information, see the [GUIDE](GUIDE.md).

## Alternatives

* Write down your test data and use a loop.
* Use the [crate `quickcheck`].
* Use the [crate `proptest`].

[crate `quickcheck`]: https://crates.io/crates/quickcheck
[crate `proptest`]: https://crates.io/crates/proptest

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
