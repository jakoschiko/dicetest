//! Hints help to analyze a single test run, mostly the counterexample.
//!
//! You can put context information like local variables
//! inside of hints. Use it to reveal what test data were generated or
//! which branches were taken. Hints must be enabled with the feature
//! `hints`.

#[cfg(feature = "hints")]
use crate::util::events;

/// A single hint that contains context information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hint {
    /// The indent level of the text.
    pub indent: usize,
    /// Contains the context information.
    pub text: String,
}

/// A collection of hints.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Hints(pub Vec<Hint>);

impl Hints {
    /// Returns an instance without any hints.
    pub fn new() -> Self {
        Hints(Vec::new())
    }
}

#[cfg(feature = "hints")]
struct InterimHints {
    current_indent: usize,
    hints: Hints,
}

#[cfg(feature = "hints")]
impl events::Events for InterimHints {
    fn new() -> Self {
        InterimHints {
            current_indent: 0,
            hints: Hints::new(),
        }
    }

    fn take(&mut self) -> Self {
        InterimHints {
            current_indent: self.current_indent,
            hints: Hints(self.hints.0.drain(..).collect()),
        }
    }
}

#[cfg(feature = "hints")]
thread_local! {
    static LOCAL: events::Stack<InterimHints> = events::new_stack();
}

/// Returns all hints that were added during the evaluation of the given function.
pub fn collect<R>(f: impl FnOnce() -> R) -> (R, Hints) {
    #[cfg(feature = "hints")]
    {
        let (result, interim) = events::collect(&LOCAL, f);
        (result, interim.hints)
    }
    #[cfg(not(feature = "hints"))]
    {
        (f(), Hints::new())
    }
}

/// Returns if hints are currently enabled.
///
/// Hints are enabled if and only if this function is executed inside of [`collect`] and
/// the feature `hints` is present.
pub fn enabled() -> bool {
    #[cfg(feature = "hints")]
    {
        events::enabled(&LOCAL)
    }
    #[cfg(not(feature = "hints"))]
    {
        false
    }
}

/// If hints are enabled, this function evaluates and adds the given hint. Otherwise this function
/// is a noop.
pub fn add(message_text: impl FnOnce() -> String) {
    #[cfg(feature = "hints")]
    {
        events::modify(&LOCAL, move |stack| {
            let text = message_text();
            let len = stack.len();

            fn add_message(interim: &mut InterimHints, text: String) {
                let indent = interim.current_indent;
                let message = Hint { indent, text };
                interim.hints.0.push(message);
            }

            stack[0..len - 1]
                .iter_mut()
                .for_each(|collection| add_message(collection, text.clone()));
            add_message(&mut stack[len - 1], text);
        });
    }
    #[cfg(not(feature = "hints"))]
    {
        drop(message_text);
    }
}

/// Increases the indent of all following added hints.
pub fn indent() {
    #[cfg(feature = "hints")]
    {
        events::modify(&LOCAL, |stack| {
            stack.iter_mut().for_each(|interim| {
                let current_indent = interim.current_indent;
                interim.current_indent = current_indent.saturating_add(1);
            });
        });
    }
}

/// Decreases the indent of all following added hints.
pub fn unindent() {
    #[cfg(feature = "hints")]
    {
        events::modify(&LOCAL, |stack| {
            stack.iter_mut().for_each(|interim| {
                let current_indent = interim.current_indent;
                interim.current_indent = current_indent.saturating_sub(1);
            });
        });
    }
}
