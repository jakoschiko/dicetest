use ::gen::Gen;
use ::gens;

/// Generates `true` or `false` with the same probability.
pub fn bool() -> impl Gen<bool> {
    gens::one_of_2(true, false)
}
