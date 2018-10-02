use ::util::{conversion, base64};
use ::rng::Rng;
use ::gen::Size;

/// The parameters for evaluating a property one time.
#[derive(Debug, Clone)]
pub struct EvalParams {
    /// The random number generator for calling `Prop::eval`.
    pub rng: Rng,
    /// The generation size for calling `Prop::eval`.
    pub size: Size,
}

impl EvalParams {
    /// Converts the `EvalParams` to a compact string representation.
    ///
    /// This function is a right inverse for `from_eval_code`.
    pub fn eval_code(&self) -> String {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.rng.seed_as_bytes());
        bytes.extend_from_slice(&conversion::u64_to_bytes(self.size.0));

        let eval_code = base64::encode(&bytes);
        eval_code
    }

    /// Tries to convert the string to `EvalParams`.
    ///
    /// The function is a left inverse for `eval_code`.
    pub fn from_eval_code(eval_code: &str) -> Result<Self, String> {
        let bytes = base64::decode(eval_code)?;

        if bytes.len() != 40 {
            return Err("Test code has invalid length".to_string());
        }

        let rng = {
            let mut seed_bytes = [0; 32];
            seed_bytes.copy_from_slice(&bytes[0..32]);
            Rng::init_with_bytes(seed_bytes)
        };

        let size = {
            let mut size_bytes = [0; 8];
            size_bytes.copy_from_slice(&bytes[32..40]);
            Size(conversion::bytes_to_u64(size_bytes))
        };

        let eval_params = EvalParams {
            rng,
            size,
        };

        Ok(eval_params)
    }
}
