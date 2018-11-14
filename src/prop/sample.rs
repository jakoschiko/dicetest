use crate::logger::Messages;
use crate::prop::Eval;

/// The result of `Prop::sample`.
pub struct Sample {
    /// The result of the property evaluation.
    pub eval: Eval,
    /// The messages that were logged during the property evaluation.
    pub messages: Messages,
}

impl Sample {
    /// Converts the `Report` to a pretty text.
    pub fn pretty(&self) -> String {
        format!(
            "Status: {:?}\nLog messages:\n{}",
            self.eval,
            self.messages.pretty()
        )
    }
}