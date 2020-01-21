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

## Prelude

Dicetest provides several preludes for different use cases:

* `dicetest::prelude::tests::*` is useful for writing tests that using `Dicetest`.
* `dicetest::prelude::dice::*` is useful for writing own generators.
* `dicetest::prelude::asserts::*` is useful for writing assertions that using generators.

## Pseudorandomness

The type `Seed` allows to determine the [pseudorandomness]. You can either use a fixed `Seed` or a random `Seed`:
```rust
use dicetest::prand::Seed;

println!("{:?}", Seed(42));
// Output: Seed(42)

println!("{:?}", Seed::random());
// Output: Seed(8019292413750407764)
```

The `Seed` can be used to initialize the [pseudorandom number generator] `Prng`. For each `Seed` the `Prng` provides a different infinite pseudorandom sequence of `u64`s
```rust
use dicetest::prand::{Prng, Seed};

fn print_random_values(mut prng: Prng) {
    for _ in 0..3 {
        print!("{:?}, ", prng.next_number());
    }
    println!("...");
}

print_random_values(Prng::from_seed(Seed(42)));
// Output: 16628028624323922065, 3476588890713931039, 59688652182557721, ...
print_random_values(Prng::from_seed(Seed(42)));
// Output: 16628028624323922065, 3476588890713931039, 59688652182557721, ...
print_random_values(Prng::from_seed(Seed::random()));
// Output: 4221507577048064061, 15374206214556255352, 4977687432463843847, ...
print_random_values(Prng::from_seed(Seed::random()));
// Output: 11086225885938422405, 9312304973013875005, 1036200222843160301, ...
```

[pseudorandomness]: https://en.wikipedia.org/wiki/Pseudorandomness
[pseudorandom number generator]: https://en.wikipedia.org/wiki/Pseudorandom_number_generator

## Dice

With `Prng` you can only generate pseudorandom `u64`s. The traits `DieOnce` and `Die` allows to implement generators for any type. You need two parameters for using them:

* `Prng` provides the pseudorandom `u64`s that the implementor of `DieOnce` or `Die` can use for constructing more complex values. The implementor should only use this as its source of randomness.
* `Limit` controls the upper size of dynamic data structures. An implementor of `DieOnce` or `Die` is allowed to freely interpret this value.

An implementor of `DieOnce` is a generator that can be used a single time (similar to `FnOnce`).
```rust
use dicetest::prelude::dice::*;

let mut prng = Prng::from_seed(0x5EED.into());
let limit = Default::default();

let xx = "xx".to_string();
let yy = "yy".to_string();

// This generator implements `DieOnce`.
// It chooses one of the `String`s without cloning them.
let xx_or_yy_die = dice::one_of_2_once(xx, yy);

println!("{:?}", xx_or_yy_die.roll_once(prng, limit));
// Output: "yy"
```

An implementor of `Die` is a generator that can be used infinite times (similar to `Fn`).
```rust
use dicetest::prelude::dice::*;

let mut prng = Prng::from_seed(0x5EED.into());
let limit = Default::default();

let xx = "xx".to_string();
let yy = "yy".to_string();

// This generator implements `Die`.
// It chooses one of the `String`s by cloning them.
let xx_or_yy_die = dice::one_of_2(xx, yy);

// This generator uses `xx_or_yy_die` to generate three `String`s at once.
let three_xx_or_yy_die = dice::array_3(xx_or_yy_die);

for _ in 0..4 {
    println!("{:?}", three_xx_or_yy_die.roll(&mut prng, limit));
}
// Output:
// ["xx", "yy", "xx"]
// ["yy", "yy", "xx"]
// ["yy", "xx", "xx"]
// ["yy", "yy", "xx"]
```

Generators can be easily implemented and composed:
```rust
use dicetest::prelude::dice::*;

// A classic die. Generates a number between 1 and 6 with uniform distribution.
let classic_die = dice::one_of_6::<u8>(1, 2, 3, 4, 5, 6);

// A loaded die. Generates the number 6 more frequently.
let loaded_die =
    dice::weighted_one_of_6::<u8>((1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (2, 6));

// Generates the result of the function.
let die_from_fn = dice::from_fn(|_| 42);

// Generates always the same `String` by cloning it.
let foo_die = dice::just("foo".to_string());

// Generates an arbitrary byte.
let byte_die = dice::u8(..);

// Generates a non-zero byte.
let non_zero_byte_die = dice::u8(1..);

// Generates a `Vec` that contains an arbitrary number of arbitrary bytes.
let bytes_die = dice::vec(dice::u8(..), ..);

// Generates a `Vec` that contains up to 10 arbitrary bytes.
let up_to_ten_bytes_die = dice::vec(dice::u8(..), ..=10);

// Generates an arbitrary wrapped byte.
struct WrappedByte(u8);
let wrapped_byte_die = dice::u8(..).map(WrappedByte);

// Generates a permutation of `(0..=n)` for an arbitrary `n`.
let permutation_die = dice::size(0..).flat_map(|n| {
    let vec = (0..=n).collect::<Vec<_>>();
    dice::shuffled_vec(vec)
});
```

Some generators can produce values of arbitrary size. But because it would be unfeasible to randomly generate a value that does not fit into the memory, the user can control the upper bound with the `Limit` parameter.
```rust
use dicetest::prelude::dice::*;

let mut prng = Prng::from_seed(0x5EED.into());
let limit = 5.into();

// Generates a `Vec` with an arbitrary length.
let vec_die = dice::vec(dice::u8(..), ..);

// Although `vec_die` can generate a `Vec` with arbitrary length,
// the actual length is bounded by `Limit`.
let vec = vec_die.roll(&mut prng, limit);

println!("{:?}", vec);
// Output: [252, 231, 153, 0]
```

Because calling `DieOnce::roll_once` and `Die::roll` with two parameters is a little bit clunky, the struct `Fate` exists to allow a more convinient usage.
```rust
use dicetest::prelude::dice::*;

let mut fate = Fate {
    prng: &mut Prng::from_seed(0x5EED.into()),
    limit: Default::default(),
};

let x = fate.roll(dice::u8(..));
let y = fate.roll(dice::f32(..));
let z = fate.roll(dice::char());
```

## Tests

For writing tests Dicetest provides a checker with the following features:
* It runs your test with different seeds.
* It logs useful information that helps when debugging your test.
* It allows configuration via source code or environment variables.

The `dicetest` macro is the most convenient way to use the checker:
```rust
use dicetest::prelude::tests::*;

#[test]
fn test_foo() {
    // Runs your test with default configuration.
    dicetest!(|fate| {
        // Write your test here.
    });
}

#[test]
fn test_bar() {
    // Runs your test with custom configuration.
    dicetest!(Config::default().with_passes(10000), |fate| {
        // Write your test here.
    });
}
```

The closure contains your test. With the passed `fate` you can generate test data and make assertions. If the closure panics, the checker catches it, logs the test result to stdout and resumes the panic.

You can use environment variables to configure how the test is run and when the test result will be logged. The documentation of [`checker::check`] lists all environment variables.

[`checker::check`]: https://docs.rs/dicetest/0.1.0/dicetest/checker/fn.check.html

## Hints

Hints can be used to analyze a single test run. In most cases you want to analyze the counterexample. Use it to reveal what test data were generated or which branches were taken:
```rust
use dicetest::prelude::tests::*;

#[test]
fn test_foo() {
    dicetest!(|mut fate| {
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
    dicetest!(|mut fate| {
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

Running the test with the environment variables `DICETEST_LOG_CONDITION=always DICETEST_STATS_ENABLED=true` produces the following output:
```text
The test withstood 1000 passes.

# Config
- seed: 9664794707732402215
- start limit: 0
- end limit: 100
- passes: 1000

# Stats
- branch:
    - 27.00% (270): if with y = 2
    - 26.50% (265): if with y = 3
    - 26.10% (261): if with y = 1
    - 20.40% (204): else
- x:
    - 28.60% (286): 1
    - 21.00% (210): 5
    - 19.70% (197): 2
    - 15.60% (156): 4
    - 15.10% (151): 3
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
DICETEST_LOG_CONDITION=always DICETEST_STATS_ENABLED=true cargo test -- --show-output mytest
```
* I want to run `mytest` with more passes and bigger test data:
```text
DICETEST_PASSES_MULTIPLIER=10 DICETEST_LIMIT_MULTIPLIER=2 cargo test mytest
```
* I want to run `mytest` with a single test run and see the test result:
```text
DICETEST_MODE=once DICETEST_LOG_CONDITION=always cargo test -- --show-output mytest
```
