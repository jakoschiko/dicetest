use crate::util::{base64, conversion};
use crate::{Limit, Prng};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunCode {
    pub prng: Prng,
    pub limit: Limit,
}

impl RunCode {
    pub fn from_base64(string: &str) -> Result<Self, String> {
        let bytes = base64::decode(string)?;

        if bytes.len() != 40 {
            return Err("Run code has invalid length".to_string());
        }

        let prng = {
            let mut seed_bytes = [0; 32];
            seed_bytes.copy_from_slice(&bytes[0..32]);
            Prng::from_bytes(seed_bytes)
        };

        let limit = {
            let mut limit_bytes = [0; 8];
            limit_bytes.copy_from_slice(&bytes[32..40]);
            Limit(conversion::bytes_to_u64(limit_bytes))
        };

        let run_code = RunCode { prng, limit };

        Ok(run_code)
    }

    pub fn to_base64(&self) -> String {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.prng.to_bytes());
        bytes.extend_from_slice(&conversion::u64_to_bytes(self.limit.0));

        base64::encode(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::RunCode;
    use crate::prelude::*;
    use crate::{asserts, Limit};

    #[test]
    fn display_is_right_inverse_for_parse() {
        Dicetest::repeatedly().run(|fate| {
            let prng_die = dice::from_fn(|fate| fate.fork_prng());
            let limit_die = dice::u64(..).map(Limit);
            let run_code_die =
                dice::zip_2(prng_die, limit_die).map(|(prng, limit)| RunCode { prng, limit });

            asserts::right_inverse(
                fate,
                run_code_die,
                |base64: String| RunCode::from_base64(&base64).unwrap(),
                |run_code| run_code.to_base64(),
            );
        })
    }
}
