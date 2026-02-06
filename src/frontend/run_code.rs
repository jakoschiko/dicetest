use std::fmt::Display;
use std::str::FromStr;

use crate::util::{base62, conversion};
use crate::{Limit, Prng};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunCode {
    pub prng: Prng,
    pub limit: Limit,
}

impl FromStr for RunCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = base62::decode(s)?;

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
}

impl Display for RunCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.prng.to_bytes());
        bytes.extend_from_slice(&conversion::u64_to_bytes(self.limit.0));

        let string = base62::encode(&bytes);

        write!(f, "{string}")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::frontend::RunCode;
    use crate::prelude::*;
    use crate::{Limit, asserts};

    #[test]
    fn to_string_is_right_inverse_for_from_str() {
        Dicetest::repeatedly().run(|fate| {
            let prng_die = dice::from_fn(|mut fate| fate.fork_prng());
            let limit_die = dice::u64(..).map(Limit);
            let run_code_die = dice::zip()
                .two(prng_die, limit_die)
                .map(|(prng, limit)| RunCode { prng, limit });

            asserts::right_inverse(
                fate,
                run_code_die,
                |base32: String| RunCode::from_str(&base32).unwrap(),
                |run_code| run_code.to_string(),
            );
        })
    }
}
