# Stats

Stats can be used to analyze multiple test runs. Use [`stat!`] to reveal the
distribution of generated test data or the probability of branches:

```rust
use dicetest::prelude::*;

#[test]
fn test_foo() {
    Dicetest::repeatedly().run(|mut fate| {
        let x = fate.roll(dice::u8(1..=5));
        stat_debug!(x);

        let y = fate.roll(dice::u8(1..=3));
        if y != x {
            stat!("branch", "if with y = {}", y)
        } else {
            stat!("branch", "else");
        }
    })
}
```

Running the test with the [environment variable] `DICETEST_STATS_ENABLED=true`
produces the following output:

```text
The test withstood 200 passes.

# Config
- seed: 5043079553183914912
- start limit: 0
- end limit: 100
- passes: 200

# Stats
- branch:
    - 29.50% (59): if with y = 1
    - 27.50% (55): if with y = 3
    - 22.50% (45): if with y = 2
    - 20.50% (41): else
- x:
    - 31.50% (63): 1
    - 22.00% (44): 5
    - 17.00% (34): 2
    - 15.50% (31): 4
    - 14.00% (28): 3
```

[`stat!`]: https://docs.rs/dicetest/latest/dicetest/macro.stat.html
[environment variable]: ./environment_variables.md
