# Guide to Dicetest

This document explains the design and the usage of Dicetest.

## Modules

Dicetest is composed of several modules:

* `prand/die/codie` These modules provides the basic types for pseudorandom value generation.
* `dice/codice` These modules provides the standard collection of generators.
* `hints/stats/runner/formatter/checker` These modules provides tools for running/debugging tests with pseudorandom test data.

```text
                     +-----------------+
       +----------+--| prand/die/codie |---+---+
       |          |  +-----------------+   |   |
       |          |                        |   |
       |          |      +-------------+   |   |
       |          |      | hints/stats |   |   |
       |          |      +-------------+   |   |
       |          |         |       |      |   |
       v          v         v       v      v   |
+-------------+  +-----------+     +--------+  |
| dice/codice |  | formatter |<----| runner |  |
+-------------+  +-----------+     +--------+  |
                     |                  |      |
                     |                  v      |
                     |         +---------+     |
                     +-------->| checker | <---+
                               +---------+
```

Dicetest provides several preludes for different use cases.

## Pseudorandomness

The type `Seed` allows to initialize the pseudorandomness:
```rust
use dicetest::prand::Seed;

println!("{:?}", Seed(42));
// Output: Seed(42)

println!("{:?}", Seed::random());
// Output: Seed(8019292413750407764)
```

The `Seed` can be used to initialize a `Prng`. For each `Seed` the `Prng` provides a different infinite pseudorandom sequence of `u64`s:
```rust
use dicetest::prand::*;

let mut prng = Prng::from_seed(Seed(42));

for _ in 0..4 {
    println!("{:?}", prng.next_number());
}
// Output:
// 16628028624323922065
// 3476588890713931039
// 59688652182557721
// 8649295813736445329
```

## Dice

With `Prng` you can only generate pseudorandom `u64`s. The traits `DieOnce` and `Die` allows to implement generators for any type.

An implementor of `DieOnce` is a generator that can be used a single time. You can generate a random value with `DieOnce::sample_once` in a quick-and-dirty way:
```rust
use dicetest::prelude::dice::*;

let xs = "xxx".to_string();
let ys = "yyy".to_string();

// This generator implements `DieOnce`.
// It chooses one of the `String`s without cloning it.
let xs_or_ys_die = dice::one_of_2_once(xs, ys);

println!("{:?}", xs_or_ys_die.sample_once());
// Output: "yyy"
```

An implementor of `Die` is a generator that can be used infinite times. you can generate a random value with `Die::sample` in a quick-and-dirty way:
```rust
use dicetest::prelude::dice::*;

let xx = "xx".to_string();
let yy = "yy".to_string();

// This generator implements `Die`.
// It chooses one of the `String`s and clones it.
let xx_or_yy_die = dice::one_of_2(xx, yy);

// This generator uses `xx_or_yy_die` three times.
let three_xx_or_yy_die = dice::array_3(xx_or_yy_die);

for _ in 0..4 {
    println!("{:?}", three_xx_or_yy_die.sample());
}
// Output:
// ["yy", "yy", "xx"]
// ["xx", "xx", "yy"]
// ["xx", "yy", "yy"]
// ["yy", "xx", "xx"]
```

Generators can be easily implemented and composed:
```rust
use dicetest::prelude::dice::*;

// A classic die. Generates a number between 1 and 6.
let classic_die = dice::u8(1..=6);

// A loaded die. Generates the number 6 more frequently.
let loaded_die = dice::weighted_one_of_6::<u8>((1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (2, 6));

// Generates the result of the function.
let die_from_fn = dice::from_fn(|_fate| 42);

// Generates always the same `String` by cloning it.
let foo_die = dice::just("foo".to_string());

struct NonZeroU8(u8);
// Generates instances of `NonZeroU8`.
let non_zero_die = dice::u8(1..).map(NonZeroU8);

// Generates permutations of `(0..=n)` for arbitrary `n`.
let permutations = dice::size(0..).flat_map(|n| {
    let vec = (0..=n).collect::<Vec<_>>();
    dice::shuffled_vec(vec)
});
```

Internally the generators uses two parameters:

* `Prng` provides the pseudorandomness. An implementor of `DieOnce` or `Die` should only use this as its source of randomness.
* `Limit` controls the maximum size of dynamic data structures. An implementor of `DieOnce` or `Die` is allowed to freely interpret this value.

The struct `Fate` contains both parameters and forbids the mutation of `Limit`. The methods `DieOnce::roll_once` and `Die::roll` take a `&mut Fate` to generate a pseudorandom value:
```rust
use dicetest::prelude::dice::*;

// The generator is allowed to mutate this `Prng`.
let mut prng = Prng::from_seed(42.into());
// But the generator cannot mutate this `Limit`.
let limit = 5.into();

let fate = &mut Fate::new(&mut prng, limit);

// Generates an arbitrary number of arbitrary bytes.
let bytes_die = dice::vec(dice::u8(..), ..);

// Nevertheless, the user of the generator can set an upper limit
// for the number of bytes. How the upper limit is interpreted
// varies from generator to generator. In this case, up to 5 bytes
// are generated.
let pseudorandom_bytes = bytes_die.roll(fate);

println!("{:?}", pseudorandom_bytes);
// Output: [2, 255, 176, 0, 92]
```

## Tests

Dicetest provides a runner for running your test with different seeds. The `dicetest` macro is the most convenient way to use the runner:
```rust
use dicetest::prelude::tests::*;

#[test]
fn test_foo() {
    // Run your test with default configuration.
    dicetest!(|fate| {
        // Your test.
    });
}

#[test]
fn test_bar() {
    // Run your test with custom configuration.
    dicetest!(Config::default().with_passes(10000), |fate| {
        // Your test.
    });
}
```

The closure contains your test. With the passed `fate` you can generate pseudorandom test data and make assertions. If the closure panics, the runner catches it, logs the test result to stdout and resumes the panic.

You can use environment variables to configure how the test is run and when the test result will be logged. The documentation of [`checker::check`] lists all environment variables.

[`checker::check`]: https://docs.rs/dicetest/0.1.0/dicetest/checker/fn.check.html

## Hints

Hints can be used to analyze a single test run. In most cases you want to analyze the counterexample. Use it to reveal what test data were generated or which branches were taken:
```rust
use dicetest::prelude::tests::*;

#[test]
fn test_foo() {
    dicetest!(|fate| {
        let x = dice::u8(1..=5).roll(fate);
        hint_debug!(x);

        let y = dice::u8(1..=3).roll(fate);
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
The test failed after 0 passes.

# Config
- seed: 1002476580450337062
- start limit: 0
- end limit: 100
- passes: 1000

# Counterexample
- run code: "wVL7vPmPphzT0HjGwTILuoksv/gH1iB0H5qfgApmKoIAAAAAAAAAAA=="
- limit: 0
- hints:
        - x = 5
        - took branch if with y = 2
- error: assertion failed: `(left == right)`
  left: `3`,
 right: `2`
```

## Stats

Stats can be used to analyze multiple test runs. Use it to reveal the distribution of generated test data or the probability of branches:
```rust
use dicetest::prelude::tests::*;

#[test]
fn test_foo() {
    dicetest!(|fate| {
        let x = dice::u8(1..=5).roll(fate);
        stat_debug!(x);

        let y = dice::u8(1..=3).roll(fate);
        if y != x {
            stat!("branch", "if with y = {}", y)
        } else {
            stat!("branch", "else");
        }
    })
}
```

Running the test with the environment variables `DICETEST_LOG_CONDITION=always DICETEST_STATS_ENABLED=true` produces the following output:
```text
Output:
The test withstood 1000 passes.

# Config
- seed: 10582207530707092664
- start limit: 0
- end limit: 100
- passes: 1000

# Stats
- branch:
        - 26% (265): if with y = 1
        - 25% (252): if with y = 3
        - 24% (243): else
        - 24% (240): if with y = 2
- x:
        - 36% (368): 1
        - 23% (237): 5
        - 20% (207): 2
        - 9% (99): 4
        - 8% (89): 3
```

## Terminal cheat sheet

* I want to debug the counterexample of `mytest` with its run code (copied from the test result):
```text
DICETEST_DEBUG=ABIDje/+CYVkmmCVTwKJ2go6VrzZWMjO2Bqc9m3b3h0DAAAAAAAAAA== cargo test mytest
```
* I want to reproduce the result of `mytest` with its seed (copied from the test result):
```text
DICETEST_SEED=795359663177100823 cargo test mytest
```
* I want to see the stats of `mytest`:
```text
DICETEST_LOG_CONDITION=always DICETEST_STATS_ENABLED=true cargo test -- --nocapture mytest
```
* I want to run `mytest` with more passes and bigger test data:
```text
DICETEST_PASSES_MULTIPLIER=10 DICETEST_LIMIT_MULTIPLIER=2 cargo test mytest
```
* I want to run `mytest` with a single test run and see the test result:
```text
DICETEST_MODE=once DICETEST_LOG_CONDITION=always cargo test -- --nocapture mytest
```
