use crate::prelude::gens::*;

/// Generates a `Ok` or a `Err` that contain a value from one of the given generators. `Ok` and
/// `Err` have the same probalility.
pub fn result_once<T, E>(
    ok_gen: impl GenOnce<T>,
    err_gen: impl GenOnce<E>,
) -> impl GenOnce<Result<T, E>> {
    gens::one_of_gen_2_once(ok_gen.map_once(Ok), err_gen.map_once(Err))
}

/// Generates a `Ok` or a `Err` that contain a value from one of the given generators. `Ok` and
/// `Err` have the same probalility.
pub fn result<T, E>(ok_gen: impl Gen<T>, err_gen: impl Gen<E>) -> impl Gen<Result<T, E>> {
    gens::one_of_gen_2(ok_gen.map(Ok), err_gen.map(Err))
}

/// Generates a `Ok` or a `Err` that contain a value from one of the given generators. The
/// probabilities of `Ok` and `Err` depend on the given weights.
pub fn weighted_result_once<T, E>(
    (ok_weight, ok_gen): (u32, impl GenOnce<T>),
    (err_weight, err_gen): (u32, impl GenOnce<E>),
) -> impl GenOnce<Result<T, E>> {
    gens::weighted_one_of_gen_2_once(
        (ok_weight, ok_gen.map_once(Ok)),
        (err_weight, err_gen.map_once(Err)),
    )
}

/// Generates a `Ok` or a `Err` that contain a value from one of the given generators.
/// The probabilities of `Ok` and `Err` depend on the given weights.
pub fn weighted_result<T, E>(
    (ok_weight, ok_gen): (u32, impl Gen<T>),
    (err_weight, err_gen): (u32, impl Gen<E>),
) -> impl Gen<Result<T, E>> {
    gens::weighted_one_of_gen_2((ok_weight, ok_gen.map(Ok)), (err_weight, err_gen.map(Err)))
}
