//! Fault classification for the stream drain loop.
//!
//! Converts a terminal error message into a [`StreamStop::Fault`], triaging
//! whether the failure looks transient (retryable) via [`fault::is_transient`].

use super::fault;
use crate::StreamStop;

/// Classify an error message into a [`StreamStop::Fault`] with a transient
/// flag, preserving the message for the final error report.
/// # Arguments
/// Receives a provider error message.
/// # Returns
/// A classified Fault stop with its display message.
/// # Examples
/// ```
/// assert!(matches!(codetether_stream_outcome::fault_from("connection reset"), codetether_stream_outcome::StreamStop::Fault { transient: true, .. }));
/// ```
pub fn fault_from(msg: &str) -> StreamStop {
    StreamStop::Fault {
        transient: fault::is_transient(msg),
        message: fault::display_message(msg).to_string(),
    }
}
