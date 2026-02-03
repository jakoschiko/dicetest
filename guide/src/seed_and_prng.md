# Seed and Prng

The type [`Seed`] allows to determine the [pseudorandomness]. You can either use a fixed
[`Seed`] or a random [`Seed`]:

```rust
use dicetest::Seed;

println!("{:?}", Seed(42));
// Output: Seed(42)

println!("{:?}", Seed::random());
// Output: Seed(8019292413750407764)
```

The [`Seed`] can be used to initialize the [pseudorandom number generator] [`Prng`]. For each
[`Seed`] the [`Prng`] provides a different infinite pseudorandom sequence of `u64`s.

```rust
use dicetest::{Prng, Seed};

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

[`Prng`]: https://docs.rs/dicetest/latest/dicetest/struct.Prng.html
[`Seed`]: https://docs.rs/dicetest/latest/dicetest/struct.Seed.html
[pseudorandom number generator]: https://en.wikipedia.org/wiki/Pseudorandom_number_generator
[pseudorandomness]: https://en.wikipedia.org/wiki/Pseudorandomness
