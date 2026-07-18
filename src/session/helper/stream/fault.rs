//! Transient-vs-permanent classification of terminal stream error messages.
//!
//! SRP only restarts on *transient* faults: network resets, gateway/5xx, and
//! rate limits, which a fresh request can plausibly clear. Permanent faults
//! (auth, malformed request, context-length, content policy) would fail
//! identically on retry and burn tokens, so they bubble straight up.

#[path = "fault/markers.rs"]
mod markers;

/// Classify a terminal error message as transient (`true`) or permanent.
///
/// Permanent markers win over transient ones so an authenticated 403 is never
/// retried even if the message also mentions a port number like 500.
pub(crate) fn is_transient(msg: &str) -> bool {
    let lower = msg.to_ascii_lowercase();
    if markers::PERMANENT.iter().any(|m| lower.contains(m)) {
        return false;
    }
    markers::TRANSIENT.iter().any(|m| lower.contains(m))
}

#[cfg(test)]
#[path = "fault_tests.rs"]
mod tests;
