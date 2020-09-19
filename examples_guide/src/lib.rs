#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(test)]
mod section_pseudorandomness {
    #[test]
    fn seed() {
        use dicetest::prand::Seed;

        println!("{:?}", Seed(42));
        // Output: Seed(42)

        println!("{:?}", Seed::random());
        // Output: Seed(8019292413750407764)
    }

    #[test]
    fn prng() {
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
    }
}

#[cfg(test)]
mod section_dice {
    #[test]
    fn die_once() {
        use dicetest::prelude::dice::*;

        let xx = "xx".to_string();
        let yy = "yy".to_string();

        // This generator implements `DieOnce`.
        // It chooses one of the `String`s without cloning them.
        let xx_or_yy_die = dice::one_of_2_once(xx, yy);
    }

    #[test]
    fn die() {
        use dicetest::prelude::dice::*;

        let xx = "xx".to_string();
        let yy = "yy".to_string();

        // This generator implements `Die`.
        // It chooses one of the `String`s by cloning them.
        let xx_or_yy_die = dice::one_of_2(xx, yy);

        // This generator uses `xx_or_yy_die` to generate three `String`s at once.
        let three_xx_or_yy_die = dice::array_3(xx_or_yy_die);
    }

    #[test]
    fn implement_and_compose() {
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
    }

    #[test]
    fn fate() {
        use dicetest::prelude::dice::*;

        // Provides the randomness for the generator and will be mutated when used.
        let mut prng = Prng::from_seed(0x5EED.into());
        // Limits the size of dynamic data structures. The generator has only read access.
        let limit = 5.into();

        // Constructs a `Fate` that is only available inside the closure.
        Fate::run(&mut prng, limit, |fate| {
            // Generates a `Vec` with an arbitrary length.
            let vec_die = dice::vec(dice::u8(..), ..);

            // Although `vec_die` can generate a `Vec` with arbitrary length,
            // the actual length is limited by `limit`.
            let vec = vec_die.roll(fate);

            println!("{:?}", vec);
            // Output: [252, 231, 153, 0]
        })
    }
}

#[cfg(test)]
mod section_tests {
    use dicetest::prelude::tests::*;

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
}

#[cfg(test)]
mod section_hints {
    use dicetest::prelude::tests::*;

    #[test]
    fn test_foo() {
        Dicetest::repeatedly().run(|fate| {
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
}

#[cfg(test)]
mod section_stats {
    use dicetest::prelude::tests::*;

    #[test]
    fn test_foo() {
        Dicetest::repeatedly().run(|fate| {
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
}
