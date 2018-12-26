use crate::prelude::gens::*;

/// Generates a `None` or a `Some` that contains a value from the given generator. `None` and `Some`
/// have the same probalility.
pub fn option_once<T>(some_gen: impl GenOnce<T>) -> impl GenOnce<Option<T>> {
    gens::one_of_gen_2_once(gens::just_once(None), some_gen.map_once(Some))
}

/// Generates a `None` or a `Some` that contains a value from the given generator. `None` and `Some`
/// have the same probalility.
pub fn option<T>(some_gen: impl Gen<T>) -> impl Gen<Option<T>> {
    gens::one_of_gen_2(gens::from_fn(|_| None), some_gen.map(Some))
}

/// Generates a `None` or a `Some` that contains a value from the given generator. The probabilities
/// of `None` and `Some` depend on the given weights.
pub fn weighted_option_once<T>(
    none_weight: u32,
    (some_weight, some_gen): (u32, impl GenOnce<T>),
) -> impl GenOnce<Option<T>> {
    gens::weighted_one_of_gen_2_once(
        (none_weight, gens::just_once(None)),
        (some_weight, some_gen.map_once(Some)),
    )
}

/// Generates a `None` or a `Some` that contains a value from the given generator. The probabilities
/// of `None` and `Some` depend on the given weights.
pub fn weighted_option<T>(
    none_weight: u32,
    (some_weight, some_gen): (u32, impl Gen<T>),
) -> impl Gen<Option<T>> {
    gens::weighted_one_of_gen_2(
        (none_weight, gens::from_fn(|_| None)),
        (some_weight, some_gen.map(Some)),
    )
}
