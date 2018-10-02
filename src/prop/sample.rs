use ::prop::{Prints, Eval};

/// The result of `Prop::sample`.
pub struct Sample {
    /// The result of the property evaluation.
    pub eval: Eval,
    /// The prints collected during the property evaluation.
    pub prints: Prints,
}

impl Sample {
    /// Converts the `Report` to a pretty text.
    pub fn pretty(&self) -> String {
        format!(
            "Status: {:?}\nPrints:\n{}",
            self.eval,
            self.prints.pretty()
        )
    }
}