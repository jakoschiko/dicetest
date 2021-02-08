use crate::prelude::*;

macro_rules! sum {
    ($h:ident,) => ($h);
    ($h:ident, $($t:ident,)*) => ($h + sum!($($t,)*));
}

macro_rules! fn_one_of_n {
    (
        $n:expr,
        $one_of_n_once:ident, $weighted_one_of_n_once:ident,
        $one_of_n:ident, $weighted_one_of_n:ident,
        $one_of_die_n_once:ident, $weighted_one_of_die_n_once:ident,
        $one_of_die_n:ident, $weighted_one_of_die_n:ident:
        $($i:expr, $weight_i:ident, $value_i:ident, $die_i:ident)+
    ) => (
        #[allow(clippy::too_many_arguments)]
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
        /// let zero_or_one = fate.roll(dice::one_of_2_once(
        ///     CannotBeCloned(0),
        ///     CannotBeCloned(1),
        /// ));
        /// assert!(zero_or_one == CannotBeCloned(0) || zero_or_one == CannotBeCloned(1));
        /// ```
        pub fn $one_of_n_once<T>(
            $($value_i: T,)*
        ) -> impl DieOnce<T> {
            $one_of_die_n_once($(dice::just_once($value_i),)*)
        }

        #[allow(clippy::too_many_arguments)]
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
        /// let loaded_die = dice::weighted_one_of_6_once(
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
        pub fn $weighted_one_of_n_once<T>(
            $(($weight_i, $value_i): (u32, T),)*
        ) -> impl DieOnce<T> {
            $weighted_one_of_die_n_once($(($weight_i, dice::just_once($value_i)),)*)
        }

        #[allow(clippy::too_many_arguments)]
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
        /// let cloning_die = dice::one_of_2(vec![0, 0], vec![1, 1]);
        /// for _ in 0..10 {
        ///     let zeroes_or_ones = fate.roll(&cloning_die);
        ///     assert!(zeroes_or_ones == vec![0, 0] || zeroes_or_ones == vec![1, 1]);
        /// }
        /// ```
        pub fn $one_of_n<T>(
            $($value_i: T,)*
        ) -> impl Die<T>
        where
            T: Clone,
        {
            $one_of_die_n($(dice::just($value_i),)*)
        }

        #[allow(clippy::too_many_arguments)]
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
        /// let loaded_die = dice::weighted_one_of_6(
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
        pub fn $weighted_one_of_n<T>(
            $(($weight_i, $value_i): (u32, T),)*
        ) -> impl Die<T>
        where
            T: Clone,
        {
            $weighted_one_of_die_n($(($weight_i, dice::just($value_i)),)*)
        }

        #[allow(clippy::too_many_arguments)]
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
        /// let zero_or_one = fate.roll(dice::one_of_die_2_once(zero_die, one_die));
        /// assert!(zero_or_one == 0 || zero_or_one == 1);
        /// ```
        pub fn $one_of_die_n_once<T>(
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

        #[allow(clippy::too_many_arguments)]
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
        /// let loaded_die = dice::weighted_one_of_die_2_once(
        ///     (1, dice::u8(1..=5)),
        ///     (6, dice::just_once(6)),
        /// );
        /// let more_often_six_than_not = fate.roll(loaded_die);
        /// assert!(more_often_six_than_not >= 0 && more_often_six_than_not <= 6);
        /// ```
        pub fn $weighted_one_of_die_n_once<T>(
            $(($weight_i, $die_i): (u32, impl DieOnce<T>),)*
        ) -> impl DieOnce<T> {
            $(let $weight_i = u64::from($weight_i);)*
            let total_weight = sum!($($weight_i,)*);
            dice::from_fn_once(move |mut fate| {
                let choice = fate.next_number() % total_weight;
                $(
                    if choice < $weight_i {
                        return fate.roll($die_i);
                    }
                    #[allow(unused_variables)]
                    let choice = choice - $weight_i;
                )*
                unreachable!()
            })
        }

        #[allow(clippy::too_many_arguments)]
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
        /// let zero_or_one_die = dice::one_of_die_2(zero_die, one_die);
        /// for _ in 0..10 {
        ///     let zero_or_one = fate.roll(&zero_or_one_die);
        ///     assert!(zero_or_one == 0 || zero_or_one == 1);
        /// }
        /// ```
        pub fn $one_of_die_n<T>(
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

        #[allow(clippy::too_many_arguments)]
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
        /// let loaded_die = dice::weighted_one_of_die_2(
        ///     (1, dice::u8(1..=5)),
        ///     (6, dice::just(6)),
        /// );
        /// for _ in 0..10 {
        ///     let more_often_six_than_not = fate.roll(&loaded_die);
        ///     assert!(more_often_six_than_not >= 0 && more_often_six_than_not <= 6);
        /// }
        /// ```
        pub fn $weighted_one_of_die_n<T>(
            $(($weight_i, $die_i): (u32, impl Die<T>),)*
        ) -> impl Die<T> {
            $(let $weight_i = u64::from($weight_i);)*
            let total_weight = sum!($($weight_i,)*);
            dice::from_fn(move |mut fate| {
                let choice = fate.next_number() % total_weight;
                $(
                    if choice < $weight_i {
                        return fate.roll(&$die_i);
                    }
                    #[allow(unused_variables)]
                    let choice = choice - $weight_i;
                )*
                unreachable!()
            })
        }
    )
}

fn_one_of_n! { 2,
    one_of_2_once, weighted_one_of_2_once,
    one_of_2, weighted_one_of_2,
    one_of_die_2_once, weighted_one_of_die_2_once,
    one_of_die_2, weighted_one_of_die_2:
    0, weight_0, value_0, die_0
    1, weight_1, value_1, die_1
}

fn_one_of_n! { 3,
    one_of_3_once, weighted_one_of_3_once,
    one_of_3, weighted_one_of_3,
    one_of_die_3_once, weighted_one_of_die_3_once,
    one_of_die_3, weighted_one_of_die_3:
    0, weight_0, value_0, die_0
    1, weight_1, value_1, die_1
    2, weight_2, value_2, die_2
}

fn_one_of_n! { 4,
    one_of_4_once, weighted_one_of_4_once,
    one_of_4, weighted_one_of_4,
    one_of_die_4_once, weighted_one_of_die_4_once,
    one_of_die_4, weighted_one_of_die_4:
    0, weight_0, value_0, die_0
    1, weight_1, value_1, die_1
    2, weight_2, value_2, die_2
    3, weight_3, value_3, die_3
}

fn_one_of_n! { 5,
    one_of_5_once, weighted_one_of_5_once,
    one_of_5, weighted_one_of_5,
    one_of_die_5_once, weighted_one_of_die_5_once,
    one_of_die_5, weighted_one_of_die_5:
    0, weight_0, value_0, die_0
    1, weight_1, value_1, die_1
    2, weight_2, value_2, die_2
    3, weight_3, value_3, die_3
    4, weight_4, value_4, die_4
}

fn_one_of_n! { 6,
    one_of_6_once, weighted_one_of_6_once,
    one_of_6, weighted_one_of_6,
    one_of_die_6_once, weighted_one_of_die_6_once,
    one_of_die_6, weighted_one_of_die_6:
    0, weight_0, value_0, die_0
    1, weight_1, value_1, die_1
    2, weight_2, value_2, die_2
    3, weight_3, value_3, die_3
    4, weight_4, value_4, die_4
    5, weight_5, value_5, die_5
}

fn_one_of_n! { 7,
    one_of_7_once, weighted_one_of_7_once,
    one_of_7, weighted_one_of_7,
    one_of_die_7_once, weighted_one_of_die_7_once,
    one_of_die_7, weighted_one_of_die_7:
    0, weight_0, value_0, die_0
    1, weight_1, value_1, die_1
    2, weight_2, value_2, die_2
    3, weight_3, value_3, die_3
    4, weight_4, value_4, die_4
    5, weight_5, value_5, die_5
    6, weight_6, value_6, die_6
}

fn_one_of_n! { 8,
    one_of_8_once, weighted_one_of_8_once,
    one_of_8, weighted_one_of_8,
    one_of_die_8_once, weighted_one_of_die_8_once,
    one_of_die_8, weighted_one_of_die_8:
    0, weight_0, value_0, die_0
    1, weight_1, value_1, die_1
    2, weight_2, value_2, die_2
    3, weight_3, value_3, die_3
    4, weight_4, value_4, die_4
    5, weight_5, value_5, die_5
    6, weight_6, value_6, die_6
    7, weight_7, value_7, die_7
}

fn_one_of_n! { 9,
    one_of_9_once, weighted_one_of_9_once,
    one_of_9, weighted_one_of_9,
    one_of_die_9_once, weighted_one_of_die_9_once,
    one_of_die_9, weighted_one_of_die_9:
    0, weight_0, value_0, die_0
    1, weight_1, value_1, die_1
    2, weight_2, value_2, die_2
    3, weight_3, value_3, die_3
    4, weight_4, value_4, die_4
    5, weight_5, value_5, die_5
    6, weight_6, value_6, die_6
    7, weight_7, value_7, die_7
    8, weight_8, value_8, die_8
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
