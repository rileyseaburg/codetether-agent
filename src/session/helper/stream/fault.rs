//! Transient-vs-permanent classification of terminal stream error messages.
//!
//! SRP only restarts on *transient* faults: network resets, gateway/5xx, and
//! rate limits, which a fresh request can plausibly clear. Permanent faults
//! (auth, malformed request, context-length, content policy) would fail
//! identically on retry and burn tokens, so they bubble straight up.

/// Substrings that mark a terminal error as permanent (never retried).
const PERMANENT_MARKERS: &[&str] = &[
    "context length",
    "context_length",
    "maximum context",
    "content filter",
    "content_policy",
    "invalid api key",
    "unauthorized",
    "forbidden",
    "400 bad request",
    "401",
    "403",
];

/// Substrings that mark a terminal error as transient (restart-eligible).
const TRANSIENT_MARKERS: &[&str] = &[
    "timeout",
    "connection reset",
    "connection closed",
    "broken pipe",
    "eof",
    "429",
    "500",
    "502",
    "503",
    "504",
    "temporarily",
];

/// Classify a terminal error message as transient (`true`) or permanent.
///
/// Permanent markers win over transient ones so an authenticated 403 is never
/// retried even if the message also mentions a port number like 500.
pub(crate) fn is_transient(msg: &str) -> bool {
    let lower = msg.to_ascii_lowercase();
    if PERMANENT_MARKERS.iter().any(|m| lower.contains(m)) {
        return false;
    }
    TRANSIENT_MARKERS.iter().any(|m| lower.contains(m))
}
