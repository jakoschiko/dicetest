use crate::util::{conversion, base64};
use crate::gen::{Prng, Limit};

/// The parameters for evaluating a property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalParams {
    pub prng: Prng,
    pub limit: Limit,
}

impl EvalParams {
    /// Converts the `EvalParams` to a compact string representation.
    ///
    /// This function is a right inverse for `from_eval_code`.
    pub fn eval_code(&self) -> String {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.prng.seed_as_bytes());
        bytes.extend_from_slice(&conversion::u64_to_bytes(self.limit.0));

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

        let prng = {
            let mut seed_bytes = [0; 32];
            seed_bytes.copy_from_slice(&bytes[0..32]);
            Prng::init_with_bytes(seed_bytes)
        };

        let limit = {
            let mut limit_bytes = [0; 8];
            limit_bytes.copy_from_slice(&bytes[32..40]);
            Limit(conversion::bytes_to_u64(limit_bytes))
        };

        let eval_params = EvalParams {
            prng,
            limit,
        };

        Ok(eval_params)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;
    use crate::gen::Limit;
    use crate::brooder::EvalParams;

    #[test]
    fn eval_code_is_right_inverse_for_from_eval_code() {
        assert_prop!({
            let eval_params_gen = gens::zip_2(
                gens::prng_fork(),
                gens::u64(..).map(Limit),
            ).map(|(prng, limit)| EvalParams { prng, limit });

            props::right_inverse(
                eval_params_gen,
                |eval_code: String| EvalParams::from_eval_code(&eval_code).unwrap(),
                |eval_params: EvalParams| eval_params.eval_code(),
            )
        });
    }
}