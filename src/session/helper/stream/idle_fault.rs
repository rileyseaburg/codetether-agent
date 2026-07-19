//! Fault classification for the stream drain loop.
//!
//! Converts a terminal error message into a [`StreamStop::Fault`], triaging
//! whether the failure looks transient (retryable) via [`fault::is_transient`].

use super::fault;
use super::outcome::StreamStop;

/// Classify an error message into a [`StreamStop::Fault`] with a transient
/// flag, preserving the message for the final error report.
pub(super) fn fault_from(msg: &str) -> StreamStop {
    StreamStop::Fault {
        transient: fault::is_transient(msg),
        message: fault::display_message(msg).to_string(),
    }
}
