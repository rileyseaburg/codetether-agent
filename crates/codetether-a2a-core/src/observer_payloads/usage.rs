//! JSON payloads for usage observations.
use super::*;

/// Build a `usage_report` observer payload.
///
/// # Arguments
/// Supply prompt_tokens, completion_tokens, duration_ms, model.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::usage_report;
/// let payload = usage_report(10, 5, 20, "model");
/// assert_eq!(payload["type"], "usage_report");
/// ```
pub fn usage_report(
    prompt_tokens: usize,
    completion_tokens: usize,
    duration_ms: u64,
    model: &str,
) -> Value {
    json!({ "type": "usage_report", "prompt_tokens": prompt_tokens, "completion_tokens": completion_tokens, "duration_ms": duration_ms, "model": model })
}
