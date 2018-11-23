/// A logged message.
#[derive(Debug, Clone)]
pub struct Message {
    /// The indention at the beginning of the text.
    pub indention: usize,
    /// The text passed to the logger.
    pub text: String,
}

/// A collection of logged messages.
#[derive(Debug, Clone)]
pub struct Messages(pub Vec<Message>);

impl Messages {
    pub fn new() -> Self {
        Messages(Vec::new())
    }

    /// Returns a `String` that contains all messages in a pretty format.
    pub fn pretty(&self) -> String {
        let mut acc = String::new();
        let mut iter = self.0.iter();

        let add_message = |acc: &mut String, message: &Message| {
            for _ in 0..message.indention {
                acc.push('\t');
            }
            acc.push_str(&message.text);
        };

        if let Some(message) = iter.next() {
            add_message(&mut acc, message);
        }

        for message in iter {
            acc.push('\n');
            add_message(&mut acc, message);
        }

        acc
    }
}
