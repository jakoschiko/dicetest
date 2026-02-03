#![allow(dead_code)]
#![allow(unused_variables)]

mod section_example {
    fn bubble_sort<T: Ord>(slice: &mut [T]) {
        let len = slice.len();

        for _ in 0..len {
            for j in 1..len - 1 {
                let jpp = j + 1;
                if slice[j] > slice[jpp] {
                    slice.swap(j, jpp);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use dicetest::prelude::*;

        #[test]
        #[should_panic]
        fn result_of_bubble_sort_is_sorted() {
            Dicetest::repeatedly().run(|mut fate| {
                let mut v = fate.roll(dice::vec(dice::u8(..), ..));
                hint!("unsorted: {:?}", v);

                bubble_sort(&mut v);
                hint!("  sorted: {:?}", v);

                let is_sorted = v.windows(2).all(|w| w[0] <= w[1]);
                assert!(is_sorted);
            })
        }
    }
}

#[cfg(test)]
mod section_pseudorandomness {
    #[test]
    fn seed() {
        use dicetest::Seed;

        println!("{:?}", Seed(42));
        // Output: Seed(42)

        println!("{:?}", Seed::random());
        // Output: Seed(8019292413750407764)
    }

    #[test]
    fn prng() {
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
    }
}

#[cfg(test)]
mod section_die_once_and_die {
    #[test]
    fn die_once() {
        use dicetest::prelude::*;

        let xx = "xx".to_string();
        let yy = "yy".to_string();

        // This generator implements `DieOnce`.
        // It chooses one of the `String`s without cloning them.
        let xx_or_yy_die = dice::one_of_once().two(xx, yy);
    }

    #[test]
    fn die() {
        use dicetest::prelude::*;

        let xx = "xx".to_string();
        let yy = "yy".to_string();

        // This generator implements `Die`.
        // It chooses one of the `String`s by cloning them.
        let xx_or_yy_die = dice::one_of().two(xx, yy);

        // This generator uses `xx_or_yy_die` to generate three `String`s at once.
        let three_xx_or_yy_die = dice::array::<_, _, 3>(xx_or_yy_die);
    }

    #[test]
    fn implement_and_compose() {
        use dicetest::prelude::*;

        // A classic die that generates a number between 1 and 6 with uniform distribution.
        let classic_die = dice::one_of().six::<u8>(1, 2, 3, 4, 5, 6);

        // A loaded die that generates the number 6 more frequently.
        let loaded_die =
            dice::weighted_one_of().six::<u8>((1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (2, 6));

        // This die generates the result of the function.
        let die_from_fn = dice::from_fn(|_| 42);

        // This die generates always the same `String` by cloning it.
        let foo_die = dice::just("foo".to_string());

        // This die generates an arbitrary byte.
        let byte_die = dice::u8(..);

        // This die generates a non-zero byte.
        let non_zero_byte_die = dice::u8(1..);

        // This die generates a `Vec` that contains an arbitrary number of arbitrary bytes.
        let bytes_die = dice::vec(dice::u8(..), ..);

        // This die generates a `Vec` that contains up to 10 arbitrary bytes.
        let up_to_ten_bytes_die = dice::vec(dice::u8(..), ..=10);

        // This die generates an arbitrary wrapped byte.
        struct WrappedByte(u8);
        let wrapped_byte_die = dice::u8(..).map(WrappedByte);

        // This die generates a permutation of `(0..=n)` for an arbitrary `n`.
        let permutation_die = dice::length(0..).flat_map(|n| {
            let vec = (0..=n).collect::<Vec<_>>();
            dice::shuffled_vec(vec)
        });
    }

    #[test]
    fn fate() {
        use dicetest::prelude::*;
        use dicetest::{Limit, Prng};

        // Provides the randomness for the generator and will be mutated when used.
        let mut prng = Prng::from_seed(0x5EED.into());
        // Limits the length of dynamic data structures. The generator has only read access.
        let limit = Limit(5);

        // Contains all parameters necessary for using `DieOnce` or `Die`.
        let mut fate = Fate::new(&mut prng, limit);

        // Generator for a `Vec` with an arbitrary length.
        let vec_die = dice::vec(dice::u8(..), ..);

        // Generates a `Vec`. Although `vec_die` can generate a `Vec` with an arbitrary length,
        // the length of the actual `Vec` is limited by `limit`.
        let vec = fate.roll(vec_die);
        assert!(vec.len() <= 5);

        println!("{:?}", vec);
        // Output: [252, 231, 153, 0]
    }
}

#[cfg(test)]
mod section_dice {
    #[test]
    fn use_dice() {
        use dicetest::prelude::*;
        use dicetest::{Limit, Prng};

        let mut prng = Prng::from_seed(0x5EED.into());
        let limit = Limit(5);
        let mut fate = Fate::new(&mut prng, limit);

        // `die()` returns a `Die` for `u8` based on `Dice`.
        let byte: u8 = fate.roll(die());
        println!("{byte:?}");
        // Output: 161
    }

    #[test]
    fn impl_dice() {
        use dicetest::prelude::*;

        struct Foo(u8);

        impl Dice for Foo {
            const USES_LIMIT: bool = false;

            fn die() -> impl dicetest::Die<Self> {
                dice::u8(..).map(Foo)
            }
        }
    }

    #[test]
    fn derive_dice() {
        use dicetest::prelude::*;

        #[derive(Dice)]
        struct Foo(u8);
    }
}

#[cfg(test)]
mod section_tests {
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
}

#[cfg(test)]
mod section_hints {
    use dicetest::prelude::*;

    #[test]
    #[should_panic]
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
}

#[cfg(test)]
mod section_stats {
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
}
