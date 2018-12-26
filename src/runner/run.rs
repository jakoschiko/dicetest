use crate::util::{conversion, base64};
use crate::gen::{Limit, Prng};

/// The configuration for running the test once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub prng: Prng,
    pub limit: Limit,
}

impl Run {
    /// Converts the `Run` to a compact string representation.
    ///
    /// This function is a right inverse for `from_run_code`.
    pub fn run_code(&self) -> String {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.prng.seed_as_bytes());
        bytes.extend_from_slice(&conversion::u64_to_bytes(self.limit.0));

        base64::encode(&bytes)
    }

    /// Tries to convert the string to `Run`.
    ///
    /// The function is a left inverse for `run_code`.
    pub fn from_run_code(run_code: &str) -> Result<Self, String> {
        let bytes = base64::decode(run_code)?;

        if bytes.len() != 40 {
            return Err("Run code has invalid length".to_string());
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

        let run = Run {
            prng,
            limit,
        };

        Ok(run)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::tests::*;
    use crate::gen::Limit;
    use crate::runner::Run;
    use crate::asserts;

    #[test]
    fn run_code_is_right_inverse_for_from_run_code() {
        dicetest!(|dice| {
            let run_gen = gens::zip_2(
                gens::prng_fork(),
                gens::u64(..).map(Limit),
            ).map(|(prng, limit)| Run { prng, limit });

            asserts::right_inverse(
                dice,
                run_gen,
                |run_code: String| Run::from_run_code(&run_code).unwrap(),
                |run: Run| run.run_code(),
            );
        })
    }
}