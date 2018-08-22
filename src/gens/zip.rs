use ::gen::{GenOnce, Gen};
use ::gens;

macro_rules! fn_zip_n {
    ($zip_n_once:ident, $zip_n:ident: $($Ti:ident, $gen_i:ident)+) => (
    	/// Generates a tuple containing the generated values of several generators.
		pub fn $zip_n_once<$($Ti,)*>(
			$($gen_i: impl GenOnce<$Ti>,)*
		) -> impl GenOnce<($($Ti,)*)> {
			gens::from_fn_once(move |rng, params| {
				($($gen_i.gen_once(rng, params),)*)
			})
		}

		/// Generates tuples containing the generated values of several generators.
		pub fn $zip_n<$($Ti,)*>(
			$($gen_i: impl Gen<$Ti>,)*
		) -> impl Gen<($($Ti,)*)> {
			gens::from_fn(move |rng, params| {
				($($gen_i.gen(rng, params),)*)
			})
		}
    )
}

fn_zip_n! { zip_2_once, zip_2:
	T1, gen_1
	T2, gen_2
}

fn_zip_n! { zip_3_once, zip_3:
	T1, gen_1
	T2, gen_2
	T3, gen_3
}

fn_zip_n! { zip_4_once, zip_4:
	T1, gen_1
	T2, gen_2
	T3, gen_3
	T4, gen_4
}

fn_zip_n! { zip_5_once, zip_5:
	T1, gen_1
	T2, gen_2
	T3, gen_3
	T4, gen_4
	T5, gen_5
}

fn_zip_n! { zip_6_once, zip_6:
	T1, gen_1
	T2, gen_2
	T3, gen_3
	T4, gen_4
	T5, gen_5
	T6, gen_6
}

fn_zip_n! { zip_7_once, zip_7:
	T1, gen_1
	T2, gen_2
	T3, gen_3
	T4, gen_4
	T5, gen_5
	T6, gen_6
	T7, gen_7
}

fn_zip_n! { zip_8_once, zip_8:
	T1, gen_1
	T2, gen_2
	T3, gen_3
	T4, gen_4
	T5, gen_5
	T6, gen_6
	T7, gen_7
	T8, gen_8
}

fn_zip_n! { zip_9_once, zip_9:
	T1, gen_1
	T2, gen_2
	T3, gen_3
	T4, gen_4
	T5, gen_5
	T6, gen_6
	T7, gen_7
	T8, gen_8
	T9, gen_9
}
