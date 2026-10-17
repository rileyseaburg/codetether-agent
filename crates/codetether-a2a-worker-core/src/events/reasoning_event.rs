//! Structured worker event reasoning event.
use super::*;

/// Builds an `agent.reasoning` payload.
///
/// Reasoning was previously never captured, so the transcript could show what
/// the agent did but never why.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::events::reasoning_event;
/// let event = reasoning_event("Checking the failing test first.");
/// assert_eq!(event["type"], serde_json::json!("agent.reasoning"));
/// ```
/// # Arguments
/// * `text` - Reasoning text to bound by the event byte limit.
/// # Returns
/// A typed reasoning event with explicit truncation metadata.
pub fn reasoning_event(text: &str) -> Value {
    let (bounded, truncated) = clamp(text, MAX_OUTPUT_BYTES);
    serde_json::json!({
        "type": "agent.reasoning",
        "payload": {
            "role": "assistant",
            "content": bounded,
            "content_truncated": truncated,
        },
    })
}
