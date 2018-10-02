use ::gen::{GenOnce, Gen};
use ::gens;

macro_rules! fn_one_of_n {
    (
        $n:expr,
        $one_of_n_once:ident, $one_of_n:ident,
        $one_of_gen_n_once:ident, $one_of_gen_n:ident:
        $($i:expr, $value_i:ident, $gen_i:ident)+
    ) => (
        /// Generates a value randomly chosen from the given values.
        pub fn $one_of_n_once<T>(
            $($value_i: T,)*
        ) -> impl GenOnce<T> {
            $one_of_gen_n_once($(gens::just_once($value_i),)*)
        }

        /// Generates a clone of a value randomly chosen from the given values.
        pub fn $one_of_n<T>(
            $($value_i: T,)*
        ) -> impl Gen<T>
        where
            T: Clone,
        {
            $one_of_gen_n($(gens::just($value_i),)*)
        }

        /// Generates a value with a randomly chosen generator.
        pub fn $one_of_gen_n_once<T>(
            $($gen_i: impl GenOnce<T>,)*
        ) -> impl GenOnce<T> {
            gens::from_fn_once(move |rng, size| {
                let choice = rng.next() % $n;
                    match choice {
                    $($i => $gen_i.gen_once(rng, size),)*
                    _ => panic!(),
                }
            })
        }

        /// Generates a value with a randomly chosen generator.
        pub fn $one_of_gen_n<T>(
            $($gen_i: impl Gen<T>,)*
        ) -> impl Gen<T>
        where
            T: Clone,
        {
            gens::from_fn(move |rng, size| {
                let choice = rng.next() % $n;
                match choice {
                    $($i => $gen_i.gen(rng, size),)*
                    _ => panic!(),
                }
            })
        }
    )
}

fn_one_of_n! { 2, one_of_2_once, one_of_2, one_of_gen_2_once, one_of_gen_2:
    0, value_1, gen_1
    1, value_2, gen_2
}

fn_one_of_n! { 3, one_of_3_once, one_of_3, one_of_gen_3_once, one_of_gen_3:
    0, value_1, gen_1
    1, value_2, gen_2
    2, value_3, gen_3
}

fn_one_of_n! { 4, one_of_4_once, one_of_4, one_of_gen_4_once, one_of_gen_4:
    0, value_1, gen_1
    1, value_2, gen_2
    2, value_3, gen_3
    3, value_4, gen_4
}

fn_one_of_n! { 5, one_of_5_once, one_of_5, one_of_gen_5_once, one_of_gen_5:
    0, value_1, gen_1
    1, value_2, gen_2
    2, value_3, gen_3
    3, value_4, gen_4
    4, value_5, gen_5
}

fn_one_of_n! { 6, one_of_6_once, one_of_6, one_of_gen_6_once, one_of_gen_6:
    0, value_1, gen_1
    1, value_2, gen_2
    2, value_3, gen_3
    3, value_4, gen_4
    4, value_5, gen_5
    5, value_6, gen_6
}

fn_one_of_n! { 7, one_of_7_once, one_of_7, one_of_gen_7_once, one_of_gen_7:
    0, value_1, gen_1
    1, value_2, gen_2
    2, value_3, gen_3
    3, value_4, gen_4
    4, value_5, gen_5
    5, value_6, gen_6
    6, value_7, gen_7
}

fn_one_of_n! { 8, one_of_8_once, one_of_8, one_of_gen_8_once, one_of_gen_8:
    0, value_1, gen_1
    1, value_2, gen_2
    2, value_3, gen_3
    3, value_4, gen_4
    4, value_5, gen_5
    5, value_6, gen_6
    6, value_7, gen_7
    7, value_8, gen_8
}

fn_one_of_n! { 9, one_of_9_once, one_of_9, one_of_gen_9_once, one_of_gen_9:
    0, value_1, gen_1
    1, value_2, gen_2
    2, value_3, gen_3
    3, value_4, gen_4
    4, value_5, gen_5
    5, value_6, gen_6
    6, value_7, gen_7
    7, value_8, gen_8
    8, value_9, gen_9
}

/// Generates a element randomly chosen from the given `Vec`.
pub fn one_of_vec_once<T>(mut values: Vec<T>) -> impl GenOnce<T> {
    gens::from_fn_once(move |rng, _| {
        let choice = (rng.next() as usize) % values.len();
        values.swap_remove(choice)
    })
}

/// Generates a clone of a element randomly chosen from the given `Vec`.
pub fn one_of_vec<T>(values: Vec<T>) -> impl Gen<T>
where
    T: Clone,
{
    gens::from_fn(move |rng, _| {
        let choice = (rng.next() as usize) % values.len();
        values[choice].clone()
    })
}

/// Generates a clone of a element randomly chosen from the given slice.
pub fn one_of_array<'a, T>(values: &'a [T]) -> impl Gen<T> + 'a
where
    T: Clone,
{
    gens::from_fn(move |rng, _| {
        let choice = (rng.next() as usize) % values.len();
        values[choice].clone()
    })
}
