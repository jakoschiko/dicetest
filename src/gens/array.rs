use ::gen::Gen;
use ::gens;

macro_rules! fn_array_n {
    ($N:expr, $array_n:ident: $($i:expr)+) => (
        /// Generates an array with random elements.
        pub fn $array_n<T>(element_gen: impl Gen<T>) -> impl Gen<[T; $N]> {
            gens::from_fn(move |rng, size| {
                [
                    $({ $i; element_gen.gen(rng, size) }, )*
                ]
            })
        }
    )
}

fn_array_n! { 1, array_1: 0 }

fn_array_n! { 2, array_2: 0 1 }

fn_array_n! { 3, array_3: 0 1 2 }

fn_array_n! { 4, array_4: 0 1 2 3 }

fn_array_n! { 5, array_5: 0 1 2 3 4 }

fn_array_n! { 6, array_6: 0 1 2 3 4 5 }

fn_array_n! { 7, array_7: 0 1 2 3 4 5 6 }

fn_array_n! { 8, array_8: 0 1 2 3 4 5 6 7 }

fn_array_n! { 9, array_9: 0 1 2 3 4 5 6 7 8 }
