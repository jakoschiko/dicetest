use crate::prelude::dice::*;

macro_rules! fn_zip_n {
    ($zip_n_once:ident, $zip_n:ident: $($Ti:ident, $die_i:ident)+) => (
        #[allow(clippy::too_many_arguments)]
        /// Generates a tuple containing the generated values of several generators.
        pub fn $zip_n_once<$($Ti,)*>(
            $($die_i: impl DieOnce<$Ti>,)*
        ) -> impl DieOnce<($($Ti,)*)> {
            dice::from_fn_once(move |fate| {
                ($($die_i.roll_once(fate),)*)
            })
        }

        #[allow(clippy::too_many_arguments)]
        /// Generates a tuple containing the generated values of several generators.
        pub fn $zip_n<$($Ti,)*>(
            $($die_i: impl Die<$Ti>,)*
        ) -> impl Die<($($Ti,)*)> {
            dice::from_fn(move |fate| {
                ($($die_i.roll(fate),)*)
            })
        }
    )
}

fn_zip_n! { zip_2_once, zip_2:
    T1, die_1
    T2, die_2
}

fn_zip_n! { zip_3_once, zip_3:
    T1, die_1
    T2, die_2
    T3, die_3
}

fn_zip_n! { zip_4_once, zip_4:
    T1, die_1
    T2, die_2
    T3, die_3
    T4, die_4
}

fn_zip_n! { zip_5_once, zip_5:
    T1, die_1
    T2, die_2
    T3, die_3
    T4, die_4
    T5, die_5
}

fn_zip_n! { zip_6_once, zip_6:
    T1, die_1
    T2, die_2
    T3, die_3
    T4, die_4
    T5, die_5
    T6, die_6
}

fn_zip_n! { zip_7_once, zip_7:
    T1, die_1
    T2, die_2
    T3, die_3
    T4, die_4
    T5, die_5
    T6, die_6
    T7, die_7
}

fn_zip_n! { zip_8_once, zip_8:
    T1, die_1
    T2, die_2
    T3, die_3
    T4, die_4
    T5, die_5
    T6, die_6
    T7, die_7
    T8, die_8
}

fn_zip_n! { zip_9_once, zip_9:
    T1, die_1
    T2, die_2
    T3, die_3
    T4, die_4
    T5, die_5
    T6, die_6
    T7, die_7
    T8, die_8
    T9, die_9
}