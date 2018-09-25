use ::gen;

/// These parameters configure the evaluation of properties. They will be passed to `Prop::eval`.
#[derive(Debug, Clone)]
pub struct Params {
    /// If set to `true`, `Label`s will be created and appended to the `Result`.
    pub create_labels: bool,
    /// Parameters for properties that uses `GenOnce`s and `Gen`s.
    pub gen_params: gen::Params,
}

impl Params {
    /// Sets the field `create_labels`.
    pub fn create_labels(self, create_labels: bool) -> Params {
        Params { create_labels, ..self }
    }

    /// Sets the field `gen_params`.
    pub fn gen_params(self, gen_params: gen::Params) -> Params {
        Params { gen_params, ..self }
    }
}

impl Default for Params {
    fn default() -> Self {
        Params {
            create_labels: true,
            gen_params: gen::Params::default(),
        }
    }
}
