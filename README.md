[![crates.io](https://img.shields.io/crates/v/dicetest.svg)](https://crates.io/crates/dicetest)
[![Documentation](https://docs.rs/dicetest/badge.svg)](https://docs.rs/dicetest)
[![Build & Test](https://github.com/jakoschiko/dicetest/actions/workflows/main.yml/badge.svg)](https://github.com/jakoschiko/dicetest/actions/workflows/main.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/jakoschiko/dicetest#License)

# dicetest

Framework for writing tests with randomly generated test data.

For more information see the [guide].

[guide]: https://jakoschiko.github.io/dicetest

## Features

These features are **available**:

- Generators for many libstd types (`u8`, `String`, `Vec`, etc.).
- Generators for functions (`FnMut`, `FnOnce`, `Fn`).
- Generator combinators (`map`, `flat_map`, `zip`, etc.).
- Derivable trait `Dice` for implementing generators.
- Configurable test runner `Dicetest`.
- Utilities for debugging tests (`hints` and `stats`).

These features are **missing**:

- Shrinking of counterexamples.
- Custom pseudorandom number generators.

## Example

Here's an example of an incorrect sort function tested with dicetest:

```rust,no_run
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
```

Running `cargo test` produces the following output:

```text
The test failed after 31 passes.

# Config
- seed: 3713861809241954222
- start limit: 0
- end limit: 100
- passes: 200

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

```rust,ignore
Dicetest::debug("/yiA1sab3S4UnCf4ozyMpxMxzg1NtFybCuYLHy0/oscDAAAAAAAAAA==").run(|mut fate| {
    // ...
})
```

After fixing the bug you can keep the counterexample as a regression test:

```rust,ignore
Dicetest::repeatedly()
    .regression("/yiA1sab3S4UnCf4ozyMpxMxzg1NtFybCuYLHy0/oscDAAAAAAAAAA==")
    .run(|mut fate| {
        // ...
    })
```

## Alternatives

- Write down your test data and use a loop.
- Use the crate [arbitrary] and [arbtest].
- Use the crate [quickcheck].
- Use the crate [proptest].

[arbitrary]: https://crates.io/crates/arbitrary
[arbtest]: https://crates.io/crates/arbtest
[quickcheck]: https://crates.io/crates/quickcheck
[proptest]: https://crates.io/crates/proptest

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
