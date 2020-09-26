use crate::prelude::*;

/// Generates a `Ok` or a `Err` that contain a value from one of the given generators. `Ok` and
/// `Err` have the same probability.
pub fn result_once<T, E>(
    ok_die: impl DieOnce<T>,
    err_die: impl DieOnce<E>,
) -> impl DieOnce<Result<T, E>> {
    dice::one_of_die_2_once(ok_die.map_once(Ok), err_die.map_once(Err))
}

/// Generates a `Ok` or a `Err` that contain a value from one of the given generators. `Ok` and
/// `Err` have the same probability.
pub fn result<T, E>(ok_die: impl Die<T>, err_die: impl Die<E>) -> impl Die<Result<T, E>> {
    dice::one_of_die_2(ok_die.map(Ok), err_die.map(Err))
}

/// Generates a `Ok` or a `Err` that contain a value from one of the given generators. The
/// probabilities of `Ok` and `Err` depend on the given weights.
pub fn weighted_result_once<T, E>(
    (ok_weight, ok_die): (u32, impl DieOnce<T>),
    (err_weight, err_die): (u32, impl DieOnce<E>),
) -> impl DieOnce<Result<T, E>> {
    dice::weighted_one_of_die_2_once(
        (ok_weight, ok_die.map_once(Ok)),
        (err_weight, err_die.map_once(Err)),
    )
}

/// Generates a `Ok` or a `Err` that contain a value from one of the given generators.
/// The probabilities of `Ok` and `Err` depend on the given weights.
pub fn weighted_result<T, E>(
    (ok_weight, ok_die): (u32, impl Die<T>),
    (err_weight, err_die): (u32, impl Die<E>),
) -> impl Die<Result<T, E>> {
    dice::weighted_one_of_die_2((ok_weight, ok_die.map(Ok)), (err_weight, err_die.map(Err)))
}
