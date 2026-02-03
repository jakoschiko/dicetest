# Dice

The trait [`Dice`] provides a [`Die`] for a type with reasonable distribution.
You can summon a [`Die`] based on [`Dice`] using the function [`die()`].

```rust
use dicetest::prelude::*;
use dicetest::{Limit, Prng};

let mut prng = Prng::from_seed(0x5EED.into());
let limit = Limit(5);
let mut fate = Fate::new(&mut prng, limit);

// `die()` returns a `Die` for `u8` based on `Dice`.
let byte: u8 = fate.roll(die());
println!("{byte:?}");
// Output: 161
```

You can [derive `Dice`]:

```rust
use dicetest::prelude::*;

#[derive(Dice)]
struct Foo(u8);
```

Or you can implement [`Dice`] manually:

```rust
use dicetest::prelude::*;

struct Foo(u8);

impl Dice for Foo {
    const USES_LIMIT: bool = false;

    fn die() -> impl dicetest::Die<Self> {
        dice::u8(..).map(Foo)
    }
}
```

[`Dice`]: https://docs.rs/dicetest/latest/dicetest/trait.Dice.html
[`Die`]: https://docs.rs/dicetest/latest/dicetest/trait.Die.html
[`die()`]: https://docs.rs/dicetest/latest/dicetest/fn.die.html
[derive `Dice`]: https://docs.rs/dicetest/latest/dicetest/derive.Dice.html
