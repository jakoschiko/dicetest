use crate::prelude::gens::*;

/// Generates the given value.
pub fn just_once<T>(value: T) -> impl GenOnce<T> {
    gens::from_fn_once(|_, _| value)
}

/// Generates a clone of the given value.
pub fn just<T>(value: T) -> impl Gen<T>
where
    T: Clone,
{
    gens::from_fn(move |_, _| value.clone())
}
