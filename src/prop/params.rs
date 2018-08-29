use ::gen;

/// These parameters configure the evaluation of properties. They will be passed to `Prop::eval`.
pub struct Params {
    /// If set to `true`, `Label`s will be created and appended to the `Result`.
    pub create_labels: bool,
    /// Parameters for properties that uses `GenOnce`s and `Gen`s.
    pub gen_params: gen::Params,
}
