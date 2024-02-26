use crate::prelude::*;

macro_rules! sum {
    ($h:ident,) => ($h);
    ($h:ident, $($t:ident,)*) => ($h + sum!($($t,)*));
}

/// Intermediate result of [`dice::one_of_once`].
///
/// [`dice::one_of_once`]: dice::one_of_once()
#[non_exhaustive]
pub struct OneOfOnceArities;

macro_rules! one_of_once_with_arity {
    ($arity:ident: $($value_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<T>(
            self,
            $($value_i: T,)*
        ) -> impl DieOnce<T> {
            one_of_die_once().$arity($(dice::just_once($value_i),)*)
        }
    )
}

impl OneOfOnceArities {
    one_of_once_with_arity! { two:
        value_0
        value_1
    }

    one_of_once_with_arity! { three:
        value_0
        value_1
        value_2
    }

    one_of_once_with_arity! { four:
        value_0
        value_1
        value_2
        value_3
    }

    one_of_once_with_arity! { five:
        value_0
        value_1
        value_2
        value_3
        value_4
    }

    one_of_once_with_arity! { six:
        value_0
        value_1
        value_2
        value_3
        value_4
        value_5
    }

    one_of_once_with_arity! { seven:
        value_0
        value_1
        value_2
        value_3
        value_4
        value_5
        value_6
    }

    one_of_once_with_arity! { eight:
        value_0
        value_1
        value_2
        value_3
        value_4
        value_5
        value_6
        value_7
    }

    one_of_once_with_arity! { nine:
        value_0
        value_1
        value_2
        value_3
        value_4
        value_5
        value_6
        value_7
        value_8
    }
}

/// Generates a value randomly chosen from the given values. All values have the same
/// probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct CannotBeCloned(u8);
/// let zero_or_one = fate.roll(dice::one_of_once().two(
///     CannotBeCloned(0),
///     CannotBeCloned(1),
/// ));
/// assert!(zero_or_one == CannotBeCloned(0) || zero_or_one == CannotBeCloned(1));
/// ```
pub fn one_of_once() -> OneOfOnceArities {
    OneOfOnceArities {}
}

/// Intermediate result of [`dice::weighted_one_of_once`].
///
/// [`dice::weighted_one_of_once`]: dice::weighted_one_of_once()
#[non_exhaustive]
pub struct WeightedOneOfOnceArities;

macro_rules! weighted_one_of_once_with_arity {
    ($arity:ident: $($weight_i:ident, $value_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<T>(
            self,
            $(($weight_i, $value_i): (u32, T),)*
        ) -> impl DieOnce<T> {
            weighted_one_of_die_once().$arity($(($weight_i, dice::just_once($value_i)),)*)
        }
    )
}

impl WeightedOneOfOnceArities {
    weighted_one_of_once_with_arity! { two:
        weight_0, value_0
        weight_1, value_1
    }

    weighted_one_of_once_with_arity! { three:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
    }

    weighted_one_of_once_with_arity! { four:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
    }

    weighted_one_of_once_with_arity! { five:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
    }

    weighted_one_of_once_with_arity! { six:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
        weight_5, value_5
    }

    weighted_one_of_once_with_arity! { seven:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
        weight_5, value_5
        weight_6, value_6
    }

    weighted_one_of_once_with_arity! { eight:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
        weight_5, value_5
        weight_6, value_6
        weight_7, value_7
    }

    weighted_one_of_once_with_arity! { nine:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
        weight_5, value_5
        weight_6, value_6
        weight_7, value_7
        weight_8, value_8
    }
}

/// Generates a value randomly chosen from the given values. The probability of a value
/// depends on its weight.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let loaded_die = dice::weighted_one_of_once().six(
///     (1, 1),
///     (1, 2),
///     (1, 3),
///     (1, 4),
///     (1, 5),
///     (6, 6),
/// );
/// let more_often_six_than_not = fate.roll(loaded_die);
/// assert!(more_often_six_than_not >= 0 && more_often_six_than_not <= 6);
/// ```
pub fn weighted_one_of_once() -> WeightedOneOfOnceArities {
    WeightedOneOfOnceArities {}
}

/// Intermediate result of [`dice::one_of`].
///
/// [`dice::one_of`]: dice::one_of()
#[non_exhaustive]
pub struct OneOfArities;

macro_rules! one_of_with_arity {
    ($arity:ident: $($value_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<T>(
            self,
            $($value_i: T,)*
        ) -> impl Die<T>
        where
            T: Clone,
        {
            one_of_die().$arity($(dice::just($value_i),)*)
        }
    )
}

impl OneOfArities {
    one_of_with_arity! { two:
        value_0
        value_1
    }

    one_of_with_arity! { three:
        value_0
        value_1
        value_2
    }

    one_of_with_arity! { four:
        value_0
        value_1
        value_2
        value_3
    }

    one_of_with_arity! { five:
        value_0
        value_1
        value_2
        value_3
        value_4
    }

    one_of_with_arity! { six:
        value_0
        value_1
        value_2
        value_3
        value_4
        value_5
    }

    one_of_with_arity! { seven:
        value_0
        value_1
        value_2
        value_3
        value_4
        value_5
        value_6
    }

    one_of_with_arity! { eight:
        value_0
        value_1
        value_2
        value_3
        value_4
        value_5
        value_6
        value_7
    }

    one_of_with_arity! { nine:
        value_0
        value_1
        value_2
        value_3
        value_4
        value_5
        value_6
        value_7
        value_8
    }
}

/// Generates a clone of a value randomly chosen from the given values. All values have the
/// same probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let cloning_die = dice::one_of().two(vec![0, 0], vec![1, 1]);
/// for _ in 0..10 {
///     let zeroes_or_ones = fate.roll(&cloning_die);
///     assert!(zeroes_or_ones == vec![0, 0] || zeroes_or_ones == vec![1, 1]);
/// }
/// ```
pub fn one_of() -> OneOfArities {
    OneOfArities {}
}

/// Intermediate result of [`dice::weighted_one_of`].
///
/// [`dice::weighted_one_of`]: dice::weighted_one_of()
#[non_exhaustive]
pub struct WeightedOneOfArities;

macro_rules! weighted_one_of_with_arity {
    ($arity:ident: $($weight_i:ident, $value_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<T>(
            self,
            $(($weight_i, $value_i): (u32, T),)*
        ) -> impl Die<T>
        where
            T: Clone,
        {
            weighted_one_of_die().$arity($(($weight_i, dice::just($value_i)),)*)
        }
    )
}

impl WeightedOneOfArities {
    weighted_one_of_with_arity! { two:
        weight_0, value_0
        weight_1, value_1
    }

    weighted_one_of_with_arity! { three:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
    }

    weighted_one_of_with_arity! { four:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
    }

    weighted_one_of_with_arity! { five:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
    }

    weighted_one_of_with_arity! { six:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
        weight_5, value_5
    }

    weighted_one_of_with_arity! { seven:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
        weight_5, value_5
        weight_6, value_6
    }

    weighted_one_of_with_arity! { eight:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
        weight_5, value_5
        weight_6, value_6
        weight_7, value_7
    }

    weighted_one_of_with_arity! { nine:
        weight_0, value_0
        weight_1, value_1
        weight_2, value_2
        weight_3, value_3
        weight_4, value_4
        weight_5, value_5
        weight_6, value_6
        weight_7, value_7
        weight_8, value_8
    }
}

/// Generates a clone of a value randomly chosen from the given values. The probability of
/// a value depends on its weight.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let loaded_die = dice::weighted_one_of().six(
///     (1, 1),
///     (1, 2),
///     (1, 3),
///     (1, 4),
///     (1, 5),
///     (6, 6),
/// );
/// for _ in 0..10 {
///     let more_often_six_than_not = fate.roll(&loaded_die);
///     assert!(more_often_six_than_not >= 0 && more_often_six_than_not <= 6);
/// }
/// ```
pub fn weighted_one_of() -> WeightedOneOfArities {
    WeightedOneOfArities {}
}

/// Intermediate result of [`dice::one_of_die_once`].
///
/// [`dice::one_of_die_once`]: dice::one_of_die_once()
#[non_exhaustive]
pub struct OneOfDieOnceArities;

macro_rules! one_of_die_once_with_arity {
    ($n:expr, $arity:ident: $($i:expr, $die_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<T>(
            self,
            $($die_i: impl DieOnce<T>,)*
        ) -> impl DieOnce<T> {
            dice::from_fn_once(move |mut fate| {
                let choice = fate.next_number() % $n;
                match choice {
                    $($i => fate.roll($die_i),)*
                    _ => panic!(),
                }
            })
        }
    )
}

impl OneOfDieOnceArities {
    one_of_die_once_with_arity! { 2, two:
        0, die_0
        1, die_1
    }

    one_of_die_once_with_arity! { 3, three:
        0, die_0
        1, die_1
        2, die_2
    }

    one_of_die_once_with_arity! { 4, four:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
    }

    one_of_die_once_with_arity! { 5, five:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
    }

    one_of_die_once_with_arity! { 6, six:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
        5, die_5
    }

    one_of_die_once_with_arity! { 7, seven:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
        5, die_5
        6, die_6
    }

    one_of_die_once_with_arity! { 8, eight:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
        5, die_5
        6, die_6
        7, die_7
    }

    one_of_die_once_with_arity! { 9, nine:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
        5, die_5
        6, die_6
        7, die_7
        8, die_8
    }
}

/// Generates a value with a randomly chosen generator. All generators have the same
/// probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let zero_die = dice::just_once(0);
/// let one_die = dice::just_once(1);
/// let zero_or_one = fate.roll(dice::one_of_die_once().two(zero_die, one_die));
/// assert!(zero_or_one == 0 || zero_or_one == 1);
/// ```
pub fn one_of_die_once() -> OneOfDieOnceArities {
    OneOfDieOnceArities {}
}

/// Intermediate result of [`dice::weighted_one_of_die_once`].
///
/// [`dice::weighted_one_of_die_once`]: dice::weighted_one_of_die_once()
#[non_exhaustive]
pub struct WeightedOneOfDieOnceArities;

macro_rules! weighted_one_of_die_once_with_arity {
    ($n:expr, $arity:ident: $($i:expr, $weight_i:ident, $die_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<T>(
            self,
            $(($weight_i, $die_i): (u32, impl DieOnce<T>),)*
        ) -> impl DieOnce<T> {
            $(let $weight_i = u64::from($weight_i);)*
            let total_weight = sum!($($weight_i,)*);
            dice::from_fn_once(move |mut fate| {
                if total_weight == 0 {
                    // All weights are 0, fall back to uniform distribution
                    let choice = fate.next_number() % $n;
                    match choice {
                        $($i => fate.roll($die_i),)*
                        _ => panic!(),
                    }
                } else {
                    let choice = fate.next_number() % total_weight;
                    $(
                        if choice < $weight_i {
                            return fate.roll($die_i);
                        }
                        #[allow(unused_variables)]
                        let choice = choice - $weight_i;
                    )*
                    unreachable!()
                }
            })
        }
    )
}

impl WeightedOneOfDieOnceArities {
    weighted_one_of_die_once_with_arity! { 2, two:
        0, weight_0, value_0
        1, weight_1, value_1
    }

    weighted_one_of_die_once_with_arity! { 3, three:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
    }

    weighted_one_of_die_once_with_arity! { 4, four:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
    }

    weighted_one_of_die_once_with_arity! { 5, five:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
    }

    weighted_one_of_die_once_with_arity! { 6, six:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
        5, weight_5, value_5
    }

    weighted_one_of_die_once_with_arity! { 7, seven:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
        5, weight_5, value_5
        6, weight_6, value_6
    }

    weighted_one_of_die_once_with_arity! { 8, eight:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
        5, weight_5, value_5
        6, weight_6, value_6
        7, weight_7, value_7
    }

    weighted_one_of_die_once_with_arity! { 9, nine:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
        5, weight_5, value_5
        6, weight_6, value_6
        7, weight_7, value_7
        8, weight_8, value_8
    }
}

/// Generates a value with a randomly chosen generator. The probability of a generator
/// depends on its weight.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let loaded_die = dice::weighted_one_of_die_once().two(
///     (1, dice::u8(1..=5)),
///     (6, dice::just_once(6)),
/// );
/// let more_often_six_than_not = fate.roll(loaded_die);
/// assert!(more_often_six_than_not >= 0 && more_often_six_than_not <= 6);
/// ```
pub fn weighted_one_of_die_once() -> WeightedOneOfDieOnceArities {
    WeightedOneOfDieOnceArities {}
}

/// Intermediate result of [`dice::one_of_die`].
///
/// [`dice::one_of`]: dice::one_of_die()
#[non_exhaustive]
pub struct OneOfDieArities;

macro_rules! one_of_die_with_arity {
    ($n:expr, $arity:ident: $($i:expr, $die_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<T>(
            self,
            $($die_i: impl Die<T>,)*
        ) -> impl Die<T> {
            dice::from_fn(move |mut fate| {
                let choice = fate.next_number() % $n;
                match choice {
                    $($i => fate.roll(&$die_i),)*
                    _ => panic!(),
                }
            })
        }
    )
}

impl OneOfDieArities {
    one_of_die_with_arity! { 2, two:
        0, die_0
        1, die_1
    }

    one_of_die_with_arity! { 3, three:
        0, die_0
        1, die_1
        2, die_2
    }

    one_of_die_with_arity! { 4, four:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
    }

    one_of_die_with_arity! { 5, five:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
    }

    one_of_die_with_arity! { 6, six:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
        5, die_5
    }

    one_of_die_with_arity! { 7, seven:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
        5, die_5
        6, die_6
    }

    one_of_die_with_arity! { 8, eight:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
        5, die_5
        6, die_6
        7, die_7
    }

    one_of_die_with_arity! { 9, nine:
        0, die_0
        1, die_1
        2, die_2
        3, die_3
        4, die_4
        5, die_5
        6, die_6
        7, die_7
        8, die_8
    }
}

/// Generates a value with a randomly chosen generator. All generators have the same
/// probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let zero_die = dice::just(0);
/// let one_die = dice::just(1);
/// let zero_or_one_die = dice::one_of_die().two(zero_die, one_die);
/// for _ in 0..10 {
///     let zero_or_one = fate.roll(&zero_or_one_die);
///     assert!(zero_or_one == 0 || zero_or_one == 1);
/// }
/// ```
pub fn one_of_die() -> OneOfDieArities {
    OneOfDieArities {}
}

/// Intermediate result of [`dice::weighted_one_of_die`].
///
/// [`dice::weighted_one_of`]: dice::weighted_one_of_die()
#[non_exhaustive]
pub struct WeightedOneOfDieArities;

macro_rules! weighted_one_of_die_with_arity {
    ($n:expr, $arity:ident: $($i:expr, $weight_i:ident, $die_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        pub fn $arity<T>(
            self,
            $(($weight_i, $die_i): (u32, impl Die<T>),)*
        ) -> impl Die<T> {
            $(let $weight_i = u64::from($weight_i);)*
            let total_weight = sum!($($weight_i,)*);
            dice::from_fn(move |mut fate| {
                if total_weight == 0 {
                    // All weights are 0, fall back to uniform distribution
                    let choice = fate.next_number() % $n;
                    match choice {
                        $($i => fate.roll(&$die_i),)*
                        _ => panic!(),
                    }
                } else {
                    let choice = fate.next_number() % total_weight;
                    $(
                        if choice < $weight_i {
                            return fate.roll(&$die_i);
                        }
                        #[allow(unused_variables)]
                        let choice = choice - $weight_i;
                    )*
                    unreachable!()
                }
            })
        }
    )
}

impl WeightedOneOfDieArities {
    weighted_one_of_die_with_arity! { 2, two:
        0, weight_0, value_0
        1, weight_1, value_1
    }

    weighted_one_of_die_with_arity! { 3, three:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
    }

    weighted_one_of_die_with_arity! { 4, four:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
    }

    weighted_one_of_die_with_arity! { 5, five:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
    }

    weighted_one_of_die_with_arity! { 6, six:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
        5, weight_5, value_5
    }

    weighted_one_of_die_with_arity! { 7, seven:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
        5, weight_5, value_5
        6, weight_6, value_6
    }

    weighted_one_of_die_with_arity! { 8, eight:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
        5, weight_5, value_5
        6, weight_6, value_6
        7, weight_7, value_7
    }

    weighted_one_of_die_with_arity! { 9, nine:
        0, weight_0, value_0
        1, weight_1, value_1
        2, weight_2, value_2
        3, weight_3, value_3
        4, weight_4, value_4
        5, weight_5, value_5
        6, weight_6, value_6
        7, weight_7, value_7
        8, weight_8, value_8
    }
}

/// Generates a value with a randomly chosen generator. The probability of a generator
/// depends on its weight.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let loaded_die = dice::weighted_one_of_die().two(
///     (1, dice::u8(1..=5)),
///     (6, dice::just(6)),
/// );
/// for _ in 0..10 {
///     let more_often_six_than_not = fate.roll(&loaded_die);
///     assert!(more_often_six_than_not >= 0 && more_often_six_than_not <= 6);
/// }
/// ```
pub fn weighted_one_of_die() -> WeightedOneOfDieArities {
    WeightedOneOfDieArities {}
}

/// Generates a element randomly chosen from the given [`Vec`]. All elements have the same
/// probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct CannotBeCloned(u8);
/// let zero_and_one = vec![CannotBeCloned(0), CannotBeCloned(1)];
/// let zero_or_one = fate.roll(dice::one_of_vec_once(zero_and_one));
/// assert!(zero_or_one == CannotBeCloned(0) || zero_or_one == CannotBeCloned(1));
/// ```
pub fn one_of_vec_once<T>(mut values: Vec<T>) -> impl DieOnce<T> {
    dice::from_fn_once(move |mut fate| {
        let choice = (fate.next_number() as usize) % values.len();
        values.swap_remove(choice)
    })
}

/// Generates a clone of a element randomly chosen from the given [`Vec`]. All elements have the
/// same probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let zeroes_and_ones = vec![vec![0, 0], vec![1, 1]];
/// let cloning_die = dice::one_of_vec(zeroes_and_ones);
/// for _ in 0..10 {
///     let zeroes_or_ones = fate.roll(&cloning_die);
///     assert!(zeroes_or_ones == vec![0, 0] || zeroes_or_ones == vec![1, 1]);
/// }
/// ```
pub fn one_of_vec<T>(values: Vec<T>) -> impl Die<T>
where
    T: Clone,
{
    dice::from_fn(move |mut fate| {
        let choice = (fate.next_number() as usize) % values.len();
        values[choice].clone()
    })
}

/// Generates a clone of a element randomly chosen from the given slice. All elements have the same
/// probability.
///
/// # Examples
///
/// ```
/// use dicetest::prelude::*;
/// use dicetest::{Prng, Limit};
///
/// let mut prng = Prng::from_seed(0x5EED.into());
/// let limit = Limit::default();
/// let mut fate = Fate::new(&mut prng, limit);
///
/// let zeroes_and_ones = [vec![0, 0], vec![1, 1]];
/// let cloning_die = dice::one_of_slice(&zeroes_and_ones);
/// for _ in 0..10 {
///     let zeroes_or_ones = fate.roll(&cloning_die);
///     assert!(zeroes_or_ones == vec![0, 0] || zeroes_or_ones == vec![1, 1]);
/// }
/// ```
pub fn one_of_slice<T>(values: &'_ [T]) -> impl Die<T> + '_
where
    T: Clone,
{
    dice::from_fn(move |mut fate| {
        let choice = (fate.next_number() as usize) % values.len();
        values[choice].clone()
    })
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn weighted_one_of_die_once_does_not_panic() {
        Dicetest::repeatedly().run(|mut fate| {
            let die = dice::just(());
            let [w1, w2, w3, w4, w5, w6, w7, w8, w9] = fate.roll(dice::array(dice::u32(..)));

            let _ = fate.roll(dice::weighted_one_of_die_once().two((w1, &die), (w2, &die)));
            let _ = fate.roll(dice::weighted_one_of_die_once().three(
                (w1, &die),
                (w2, &die),
                (w3, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die_once().four(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die_once().five(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die_once().six(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
                (w6, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die_once().seven(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
                (w6, &die),
                (w7, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die_once().eight(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
                (w6, &die),
                (w7, &die),
                (w8, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die_once().nine(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
                (w6, &die),
                (w7, &die),
                (w8, &die),
                (w9, &die),
            ));
        })
    }

    #[test]
    fn weighted_one_of_die_does_not_panic() {
        Dicetest::repeatedly().run(|mut fate| {
            let die = dice::just(());
            let [w1, w2, w3, w4, w5, w6, w7, w8, w9] = fate.roll(dice::array(dice::u32(..)));

            let _ = fate.roll(dice::weighted_one_of_die().two((w1, &die), (w2, &die)));
            let _ =
                fate.roll(dice::weighted_one_of_die().three((w1, &die), (w2, &die), (w3, &die)));
            let _ = fate.roll(dice::weighted_one_of_die().four(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die().five(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die().six(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
                (w6, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die().seven(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
                (w6, &die),
                (w7, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die().eight(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
                (w6, &die),
                (w7, &die),
                (w8, &die),
            ));
            let _ = fate.roll(dice::weighted_one_of_die().nine(
                (w1, &die),
                (w2, &die),
                (w3, &die),
                (w4, &die),
                (w5, &die),
                (w6, &die),
                (w7, &die),
                (w8, &die),
                (w9, &die),
            ));
        })
    }
}
