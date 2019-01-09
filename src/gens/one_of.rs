use crate::prelude::gens::*;

macro_rules! sum {
    ($h:ident,) => ($h);
    ($h:ident, $($t:ident,)*) => ($h + sum!($($t,)*));
}

macro_rules! fn_one_of_n {
    (
        $n:expr,
        $one_of_n_once:ident, $weighted_one_of_n_once:ident,
        $one_of_n:ident, $weighted_one_of_n:ident,
        $one_of_gen_n_once:ident, $weighted_one_of_gen_n_once:ident,
        $one_of_gen_n:ident, $weighted_one_of_gen_n:ident:
        $($i:expr, $weight_i:ident, $value_i:ident, $gen_i:ident)+
    ) => (
        #[allow(clippy::too_many_arguments)]
        /// Generates a value randomly chosen from the given values. All values have the same
        /// probability.
        pub fn $one_of_n_once<T>(
            $($value_i: T,)*
        ) -> impl GenOnce<T> {
            $one_of_gen_n_once($(gens::just_once($value_i),)*)
        }

        #[allow(clippy::too_many_arguments)]
        /// Generates a value randomly chosen from the given values. The probability of a value
        /// depends on its weight.
        pub fn $weighted_one_of_n_once<T>(
            $(($weight_i, $value_i): (u32, T),)*
        ) -> impl GenOnce<T> {
            $weighted_one_of_gen_n_once($(($weight_i, gens::just_once($value_i)),)*)
        }

        #[allow(clippy::too_many_arguments)]
        /// Generates a clone of a value randomly chosen from the given values. All values have the
        /// same probability.
        pub fn $one_of_n<T>(
            $($value_i: T,)*
        ) -> impl Gen<T>
        where
            T: Clone,
        {
            $one_of_gen_n($(gens::just($value_i),)*)
        }

        #[allow(clippy::too_many_arguments)]
        /// Generates a clone of a value randomly chosen from the given values. The probability of
        /// a value depends on its weight.
        pub fn $weighted_one_of_n<T>(
            $(($weight_i, $value_i): (u32, T),)*
        ) -> impl Gen<T>
        where
            T: Clone,
        {
            $weighted_one_of_gen_n($(($weight_i, gens::just($value_i)),)*)
        }

        #[allow(clippy::too_many_arguments)]
        /// Generates a value with a randomly chosen generator. All generators have the same
        /// probability.
        pub fn $one_of_gen_n_once<T>(
            $($gen_i: impl GenOnce<T>,)*
        ) -> impl GenOnce<T> {
            gens::from_fn_once(move |fate| {
                let choice = fate.prng.next_number() % $n;
                    match choice {
                    $($i => $gen_i.gen_once(fate),)*
                    _ => panic!(),
                }
            })
        }

        #[allow(clippy::too_many_arguments)]
        /// Generates a value with a randomly chosen generator. The probability of a generator
        /// depends on its weight.
        pub fn $weighted_one_of_gen_n_once<T>(
            $(($weight_i, $gen_i): (u32, impl GenOnce<T>),)*
        ) -> impl GenOnce<T> {
            $(let $weight_i = u64::from($weight_i);)*
            let total_weight = sum!($($weight_i,)*);
            gens::from_fn_once(move |fate| {
                let choice = fate.prng.next_number() % total_weight;
                $(
                    if choice < $weight_i {
                        return $gen_i.gen_once(fate);
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
        pub fn $one_of_gen_n<T>(
            $($gen_i: impl Gen<T>,)*
        ) -> impl Gen<T> {
            gens::from_fn(move |fate| {
                let choice = fate.prng.next_number() % $n;
                match choice {
                    $($i => $gen_i.gen(fate),)*
                    _ => panic!(),
                }
            })
        }

        #[allow(clippy::too_many_arguments)]
        /// Generates a value with a randomly chosen generator. The probability of a generator
        /// depends on its weight.
        pub fn $weighted_one_of_gen_n<T>(
            $(($weight_i, $gen_i): (u32, impl Gen<T>),)*
        ) -> impl Gen<T> {
            $(let $weight_i = u64::from($weight_i);)*
            let total_weight = sum!($($weight_i,)*);
            gens::from_fn(move |fate| {
                let choice = fate.prng.next_number() % total_weight;
                $(
                    if choice < $weight_i {
                        return $gen_i.gen(fate);
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
    one_of_gen_2_once, weighted_one_of_gen_2_once,
    one_of_gen_2, weighted_one_of_gen_2:
    0, weight_0, value_0, gen_0
    1, weight_1, value_1, gen_1
}

fn_one_of_n! { 3,
    one_of_3_once, weighted_one_of_3_once,
    one_of_3, weighted_one_of_3,
    one_of_gen_3_once, weighted_one_of_gen_3_once,
    one_of_gen_3, weighted_one_of_gen_3:
    0, weight_0, value_0, gen_0
    1, weight_1, value_1, gen_1
    2, weight_2, value_2, gen_2
}

fn_one_of_n! { 4,
    one_of_4_once, weighted_one_of_4_once,
    one_of_4, weighted_one_of_4,
    one_of_gen_4_once, weighted_one_of_gen_4_once,
    one_of_gen_4, weighted_one_of_gen_4:
    0, weight_0, value_0, gen_0
    1, weight_1, value_1, gen_1
    2, weight_2, value_2, gen_2
    3, weight_3, value_3, gen_3
}

fn_one_of_n! { 5,
    one_of_5_once, weighted_one_of_5_once,
    one_of_5, weighted_one_of_5,
    one_of_gen_5_once, weighted_one_of_gen_5_once,
    one_of_gen_5, weighted_one_of_gen_5:
    0, weight_0, value_0, gen_0
    1, weight_1, value_1, gen_1
    2, weight_2, value_2, gen_2
    3, weight_3, value_3, gen_3
    4, weight_4, value_4, gen_4
}

fn_one_of_n! { 6,
    one_of_6_once, weighted_one_of_6_once,
    one_of_6, weighted_one_of_6,
    one_of_gen_6_once, weighted_one_of_gen_6_once,
    one_of_gen_6, weighted_one_of_gen_6:
    0, weight_0, value_0, gen_0
    1, weight_1, value_1, gen_1
    2, weight_2, value_2, gen_2
    3, weight_3, value_3, gen_3
    4, weight_4, value_4, gen_4
    5, weight_5, value_5, gen_5
}

fn_one_of_n! { 7,
    one_of_7_once, weighted_one_of_7_once,
    one_of_7, weighted_one_of_7,
    one_of_gen_7_once, weighted_one_of_gen_7_once,
    one_of_gen_7, weighted_one_of_gen_7:
    0, weight_0, value_0, gen_0
    1, weight_1, value_1, gen_1
    2, weight_2, value_2, gen_2
    3, weight_3, value_3, gen_3
    4, weight_4, value_4, gen_4
    5, weight_5, value_5, gen_5
    6, weight_6, value_6, gen_6
}

fn_one_of_n! { 8,
    one_of_8_once, weighted_one_of_8_once,
    one_of_8, weighted_one_of_8,
    one_of_gen_8_once, weighted_one_of_gen_8_once,
    one_of_gen_8, weighted_one_of_gen_8:
    0, weight_0, value_0, gen_0
    1, weight_1, value_1, gen_1
    2, weight_2, value_2, gen_2
    3, weight_3, value_3, gen_3
    4, weight_4, value_4, gen_4
    5, weight_5, value_5, gen_5
    6, weight_6, value_6, gen_6
    7, weight_7, value_7, gen_7
}

fn_one_of_n! { 9,
    one_of_9_once, weighted_one_of_9_once,
    one_of_9, weighted_one_of_9,
    one_of_gen_9_once, weighted_one_of_gen_9_once,
    one_of_gen_9, weighted_one_of_gen_9:
    0, weight_0, value_0, gen_0
    1, weight_1, value_1, gen_1
    2, weight_2, value_2, gen_2
    3, weight_3, value_3, gen_3
    4, weight_4, value_4, gen_4
    5, weight_5, value_5, gen_5
    6, weight_6, value_6, gen_6
    7, weight_7, value_7, gen_7
    8, weight_8, value_8, gen_8
}

/// Generates a element randomly chosen from the given `Vec`. All elements have the same
/// probability.
pub fn one_of_vec_once<T>(mut values: Vec<T>) -> impl GenOnce<T> {
    gens::from_fn_once(move |fate| {
        let choice = (fate.prng.next_number() as usize) % values.len();
        values.swap_remove(choice)
    })
}

/// Generates a clone of a element randomly chosen from the given `Vec`. All elements have the same
/// probability.
pub fn one_of_vec<T>(values: Vec<T>) -> impl Gen<T>
where
    T: Clone,
{
    gens::from_fn(move |fate| {
        let choice = (fate.prng.next_number() as usize) % values.len();
        values[choice].clone()
    })
}

/// Generates a clone of a element randomly chosen from the given slice. All elements have the same
/// probability.
pub fn one_of_array<'a, T>(values: &'a [T]) -> impl Gen<T> + 'a
where
    T: Clone,
{
    gens::from_fn(move |fate| {
        let choice = (fate.prng.next_number() as usize) % values.len();
        values[choice].clone()
    })
}
