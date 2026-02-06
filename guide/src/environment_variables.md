# Environment variables

You can use environment variables to configure your tests without changing the source code.
See the documentation of [`Dicetest`] for a full list of supported environment variables.
Here are some examples:

You want to debug the counterexample of `mytest` with its run code (copied from the test result):

```text
DICETEST_DEBUG=3lTBtDxQx6SneW3r4sNLUVoYAREJ8OuO9B0yp31nna0NdwFGFvA4no cargo test mytest
```

You want to reproduce the result of `mytest` with its seed (copied from the test result):

```text
DICETEST_SEED=795359663177100823 cargo test mytest
```

You want to see the stats of `mytest`:

```text
DICETEST_STATS_ENABLED=true cargo test -- --show-output mytest
```

You want to run `mytest` with more passes and bigger test data:

```text
DICETEST_PASSES_MULTIPLIER=10 DICETEST_LIMIT_MULTIPLIER=2 cargo test mytest
```

You want to run `mytest` with a single test run and see the test result:

```text
DICETEST_MODE=once cargo test -- --show-output mytest
```

[`Dicetest`]: https://docs.rs/dicetest/latest/dicetest/struct.Dicetest.html
