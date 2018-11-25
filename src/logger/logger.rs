#[cfg(not(feature = "disabled_logger"))]
use std::cell::RefCell;

#[cfg(not(feature = "disabled_logger"))]
use crate::util::Finalizer;
#[cfg(not(feature = "disabled_logger"))]
use crate::logger::Message;
use crate::logger::Messages;

#[cfg(not(feature = "disabled_logger"))]
struct Collection {
    current_indention: usize,
    messages: Messages,
}

#[cfg(not(feature = "disabled_logger"))]
thread_local! {
    static LOCAL: RefCell<Vec<Collection>> = RefCell::new(Vec::new());
}

/// Returns all messages that were logged during the evaluation of `f`.
pub fn collect_messages<R>(f: impl FnOnce() -> R) -> (R, Messages) {
    #[cfg(not(feature = "disabled_logger"))]
    {
        LOCAL.with(move |cell| {
            {
                let mut collections = cell.borrow_mut();
                let collection = Collection {
                    messages: Messages::new(),
                    current_indention: 0,
                };
                collections.push(collection);
            }

            // Removes the collection even in case of panic
            let finalizer = Finalizer::new(|| {
                LOCAL.with(move |cell| {
                    let mut collections = cell.borrow_mut();
                    collections.pop();
                })
            });

            // This function may panic
            let result = f();

            let messages = {
                let mut collections = cell.borrow_mut();
                let collection = collections.last_mut().unwrap();
                collection.messages.take()
            };

            drop(finalizer);

            (result, messages)
        })
    }
    #[cfg(feature = "disabled_logger")]
    {
        (f(), Messages::new())
    }
}

#[cfg(not(feature = "disabled_logger"))]
fn enabled_with_cell(cell: &RefCell<Vec<Collection>>) -> bool {
    !cell.borrow().is_empty()
}

/// Returns if the logger is currently enabled.
///
/// The logger is enabled iff this function is executed inside of `collect_messages` and
/// the feature `disabled_logger` is not present.
pub fn enabled() -> bool {
    #[cfg(not(feature = "disabled_logger"))]
    {
        LOCAL.with(move |cell| enabled_with_cell(&cell))
    }
    #[cfg(feature = "disabled_logger")]
    {
        false
    }
}

/// Evaluates and logs the given message iff the logger is enabled.
pub fn log(message_text: impl FnOnce() -> String) {
    #[cfg(not(feature = "disabled_logger"))]
    {
        LOCAL.with(move |cell| {
            if enabled_with_cell(&cell) {
                let mut collections = cell.borrow_mut();
                let text = message_text();
                let len = collections.len();

                fn add_message(collection: &mut Collection, text: String) {
                    let indention = collection.current_indention;
                    let message = Message { indention, text };
                    collection.messages.0.push(message);
                }

                collections[0..len-1]
                    .iter_mut()
                    .for_each(|collection| add_message(collection, text.clone()));
                add_message(&mut collections[len-1], text);
            }
        });
    }
    #[cfg(feature = "disabled_logger")]
    {
        drop(message_text)
    }
}

/// Increases the indention of all following logged messages.
pub fn indent() {
    #[cfg(not(feature = "disabled_logger"))]
    {
        LOCAL.with(move |cell| {
            if enabled_with_cell(&cell) {
                let mut collections = cell.borrow_mut();
                collections
                    .iter_mut()
                    .for_each(|collection| {
                        let current_indention = collection.current_indention;
                        collection.current_indention = current_indention.saturating_add(1);
                    });
            }
        })
    }
}

/// Decreases the indention of all following logged messages.
pub fn unindent() {
    #[cfg(not(feature = "disabled_logger"))]
    {
        LOCAL.with(move |cell| {
            if enabled_with_cell(&cell) {
                let mut collections = cell.borrow_mut();
                collections
                    .iter_mut()
                    .for_each(|collection| {
                        let current_indention = collection.current_indention;
                        collection.current_indention = current_indention.saturating_sub(1);
                    });
            }
        })
    }
}

#[cfg(not(feature = "disabled_logger"))]
#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    use crate::logger;

    #[test]
    fn logger_is_only_enabled_during_collection() {
        assert!(!logger::enabled());
        logger::collect_messages(|| {
            assert!(logger::enabled());
        });
        assert!(!logger::enabled());
    }

    #[test]
    fn logger_is_not_enabled_after_panic() {
        let _ = catch_unwind(|| {
            logger::collect_messages(|| {
                panic!();
            })
        });
        assert!(!logger::enabled());
    }
}

#[cfg(feature = "disabled_logger")]
#[cfg(test)]
mod tests {
    use crate::logger;

    #[test]
    fn logger_is_never_enabled() {
        assert!(!logger::enabled());
        logger::collect_messages(|| {
            assert!(!logger::enabled());
        });
        assert!(!logger::enabled());
    }
}