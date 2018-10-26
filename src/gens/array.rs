use crate::prelude::gens::*;

macro_rules! fn_array_n {
    ($N:expr, $array_n:ident: $($i:expr)+) => (
        /// Generates an array with random elements.
        pub fn $array_n<T>(element_gen: impl Gen<T>) -> impl Gen<[T; $N]> {
            gens::from_fn(move |rng, lim| {
                [
                    $({ $i; element_gen.gen(rng, lim) }, )*
                ]
            })
        }
    )
}

fn_array_n! { 1, array_1:
    0
}

fn_array_n! { 2, array_2:
    0 1
}

fn_array_n! { 3, array_3:
    0 1 2
}

fn_array_n! { 4, array_4:
    0 1 2 3
}

fn_array_n! { 5, array_5:
    0 1 2 3 4
}

fn_array_n! { 6, array_6:
    0 1 2 3 4 5
}

fn_array_n! { 7, array_7:
    0 1 2 3 4 5 6
}

fn_array_n! { 8, array_8:
    0 1 2 3 4 5 6 7
}

fn_array_n! { 9, array_9:
    0 1 2 3 4 5 6 7 8
}

fn_array_n! { 10, array_10:
    0 1 2 3 4 5 6 7 8 9
}

fn_array_n! { 11, array_11:
    0 1 2 3 4 5 6 7 8 9
    10
}

fn_array_n! { 12, array_12:
    0 1 2 3 4 5 6 7 8 9
    10 11
}

fn_array_n! { 13, array_13:
    0 1 2 3 4 5 6 7 8 9
    10 11 12
}

fn_array_n! { 14, array_14:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13
}

fn_array_n! { 15, array_15:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14
}

fn_array_n! { 16, array_16:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15
}

fn_array_n! { 17, array_17:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16
}

fn_array_n! { 18, array_18:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17
}

fn_array_n! { 19, array_19:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18
}

fn_array_n! { 20, array_20:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
}

fn_array_n! { 21, array_21:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20
}

fn_array_n! { 22, array_22:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21
}

fn_array_n! { 23, array_23:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22
}

fn_array_n! { 24, array_24:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23
}

fn_array_n! { 25, array_25:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24
}

fn_array_n! { 26, array_26:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25
}

fn_array_n! { 27, array_27:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26
}

fn_array_n! { 28, array_28:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27
}

fn_array_n! { 29, array_29:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28
}

fn_array_n! { 30, array_30:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
}

fn_array_n! { 31, array_31:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30
}

fn_array_n! { 32, array_32:
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31
}
