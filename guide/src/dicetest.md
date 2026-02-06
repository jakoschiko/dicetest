# Dicetest

If you want to write a test with randomly generated test data you can use the test
builder [`Dicetest`] which has the following features:

- It can be configured via source code or [environment variables].
- It runs your test repeatedly with different seeds.
- It logs useful information that helps you to debug your test.

You can use [`Dicetest`] as part of normal unit tests:

```rust
use dicetest::prelude::*;

#[test]
fn test_foo() {
    // Runs your test with default configuration.
    Dicetest::repeatedly().run(|fate| {
        // Write your test here.
    });
}

#[test]
fn test_bar() {
    // Runs your test with custom configuration.
    Dicetest::repeatedly().passes(10000).run(|fate| {
        // Write your test here.
    });
}
```

The closure contains your test. With the passed `fate` you can generate test
data and make assertions. If the closure panics, [`Dicetest`] catches the panic,
logs the test result to stdout and resumes the panic.

The logged test result shows a run code. It identifies the counterexample
that has panicked. You can rerun the counterexample by setting an
[environment variable]:

```text
DICETEST_DEBUG=3lTBtDxQx6SneW3r4sNLUVoYAREJ8OuO9B0yp31nna0NdwFGFvA4no cargo test
```

Or you can modify the test:

```rust,ignore
Dicetest::debug("3lTBtDxQx6SneW3r4sNLUVoYAREJ8OuO9B0yp31nna0NdwFGFvA4no").run(|mut fate| {
    // ...
})
```

After fixing the bug you can keep the counterexample as a regression test:

```rust,ignore
Dicetest::repeatedly()
    .regression("3lTBtDxQx6SneW3r4sNLUVoYAREJ8OuO9B0yp31nna0NdwFGFvA4no")
    .run(|mut fate| {
        // ...
    })
```

[`Dicetest`]: https://docs.rs/dicetest/latest/dicetest/struct.Dicetest.html
[environment variable]: ./environment_variables.md
[environment variables]: ./environment_variables.md
