# Hints

Hints can be used to analyze a single test run. In most cases you want to
analyze the counterexample. Use [`hint!`] to reveal what test data were generated
or which branches were taken:

```rust
use dicetest::prelude::*;

#[test]
fn test_foo() {
    Dicetest::repeatedly().run(|mut fate| {
        let x = fate.roll(dice::u8(1..=5));
        hint_debug!(x);

        let y = fate.roll(dice::u8(1..=3));
        if y != x {
            hint!("took branch if with y = {}", y);

            assert_eq!(3, y);
        } else {
            hint!("took branch else");
        }
    })
}
```

Running the test produces the following output:

```text
The test failed after 1 passes.

# Config
- seed: 5786451887221281880
- start limit: 0
- end limit: 100
- passes: 200

# Counterexample
- run code: sjlW1FE9iGhSktxoMwGUKtFbihHPVjvQpzMObGPTqg571TEYz2rB2
- limit: 0
- hints:
        - x = 5
        - took branch if with y = 1
- error: assertion `left == right` failed
  left: 3
 right: 1
```

[`hint!`]: https://docs.rs/dicetest/latest/dicetest/macro.hint.html
