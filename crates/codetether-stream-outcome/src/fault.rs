//! Transient-vs-permanent classification of terminal stream error messages.
//!
//! SRP only restarts on *transient* faults: network resets, gateway/5xx, and
//! rate limits, which a fresh request can plausibly clear. Permanent faults
//! (auth, malformed request, context-length, content policy) would fail
//! identically on retry and burn tokens, so they bubble straight up.

use super::markers;

/// Classify a terminal error message as transient (`true`) or permanent.
///
/// Permanent markers win over transient ones so an authenticated 403 is never
/// retried even if the message also mentions a port number like 500.
/// # Arguments
/// Receives a raw provider error message.
/// # Returns
/// Whether a fresh request may recover; permanent markers override generic transient text.
/// # Examples
/// ```
/// assert!(!codetether_stream_outcome::is_transient("403 forbidden after 500ms"));
/// ```
pub fn is_transient(msg: &str) -> bool {
    let lower = msg.to_ascii_lowercase();
    if lower.starts_with("codex-retryable: ") {
        return true;
    }
    if lower.starts_with("codex-permanent: ") {
        return false;
    }
    if lower.starts_with("zai-retryable: ") {
        return true;
    }
    if lower.starts_with("zai-permanent: ") {
        return false;
    }
    if markers::PERMANENT.iter().any(|m| lower.contains(m)) {
        return false;
    }
    markers::TRANSIENT.iter().any(|m| lower.contains(m))
}

/// # Arguments
/// Receives a raw provider error message.
/// # Returns
/// The message with recognized provider classification prefixes removed.
/// # Examples
/// ```
/// assert_eq!(codetether_stream_outcome::display_message("codex-retryable: reset"), "reset");
/// ```
pub fn display_message(msg: &str) -> &str {
    msg.strip_prefix("codex-retryable: ")
        .or_else(|| msg.strip_prefix("codex-permanent: "))
        .or_else(|| msg.strip_prefix("zai-retryable: "))
        .or_else(|| msg.strip_prefix("zai-permanent: "))
        .unwrap_or(msg)
}
