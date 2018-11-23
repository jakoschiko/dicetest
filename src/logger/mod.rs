//! Provides a logger that stores messages in thread-local memory.
//!
//! Can be completely disabled with the feature `disabled_logger`.

mod message;
pub use self::message::{Message, Messages};

mod logger;
pub use self::logger::*;