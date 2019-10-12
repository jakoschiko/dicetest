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
        use dicetest::prand::*;

        let mut prng = Prng::from_seed(Seed(42));

        for _ in 0..4 {
            println!("{:?}", prng.next_number());
        }
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

        println!("{:?}", xx_or_yy_die.sample_once());
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

        for _ in 0..4 {
            println!("{:?}", three_xx_or_yy_die.sample());
        }
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
        let die_from_fn = dice::from_fn(|_fate| 42);

        // Generates always the same `String` by cloning it.
        let foo_die = dice::just("foo".to_string());

        // Generates arbitrary bytes.
        let byte_die = dice::u8(..);

        // Generates non-zero bytes.
        let non_zero_byte_die = dice::u8(1..);

        // Generates an arbitrary number of arbitrary bytes.
        let bytes_die = dice::vec(dice::u8(..), ..);

        // Generates up to 10 arbitrary bytes.
        let up_to_ten_bytes_die = dice::vec(dice::u8(..), ..=10);

        // Generates arbitrary wrapped bytes.
        struct WrappedByte(u8);
        let wrapped_byte_die = dice::u8(..).map(WrappedByte);

        // Generates permutations of `(0..=n)` for arbitrary `n`.
        let permutations_die = dice::size(0..).flat_map(|n| {
            let vec = (0..=n).collect::<Vec<_>>();
            dice::shuffled_vec(vec)
        });
    }

    #[test]
    fn fate() {
        use dicetest::prelude::dice::*;

        // The generator is allowed to mutate this `Prng`.
        let mut prng: Prng = Prng::from_seed(42.into());
        // But the generator cannot mutate this `Limit`.
        let limit: Limit = 5.into();

        let fate: &mut Fate = &mut Fate::new(&mut prng, limit);

        // Generator for an arbitrary number of bytes.
        let bytes_die = dice::vec(dice::u8(..), ..);

        // Although `bytes_die` can generate an arbitrary number of bytes,
        // the `Limit` is used as an upper limit. How the upper limit is
        // interpreted varies from generator to generator. In this case,
        // up to 5 bytes are generated.
        let bytes = bytes_die.roll(fate);

        println!("{:?}", bytes);
    }
}

#[cfg(test)]
mod section_tests {
    use dicetest::prelude::tests::*;

    #[test]
    fn test_foo() {
        // Runs your test with default configuration.
        dicetest!(|fate| {
            // Your test.
        });
    }

    #[test]
    fn test_bar() {
        // Runs your test with custom configuration.
        dicetest!(Config::default().with_passes(10000), |fate| {
            // Your test.
        });
    }
}

#[cfg(test)]
mod section_hints {
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
}

#[cfg(test)]
mod section_stats {
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
}
