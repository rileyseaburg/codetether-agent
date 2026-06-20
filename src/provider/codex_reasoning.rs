//! Reasoning-event forwarding for the OpenAI Codex Responses SSE stream.
//!
//! The Responses API emits `response.reasoning_*` events while the model is
//! thinking server-side, often for far longer than the TUI watchdog timeout
//! and *before* any visible `output_text` delta or function call. Those events
//! were previously dropped by the SSE parser's catch-all arm, so the session
//! layer saw no activity and the watchdog falsely declared the request stalled
//! (the "no events for 60s" auto-restart loop).
//!
//! Forwarding them as [`StreamChunk::Thinking`] keeps the watchdog alive: the
//! stream drain emits a [`SessionEvent::Thinking`] tick on every thinking
//! chunk, refreshing `main_last_event_at`.
//!
//! [`SessionEvent::Thinking`]: crate::session::SessionEvent::Thinking

use crate::provider::StreamChunk;
use serde_json::Value;

/// Push a [`StreamChunk::Thinking`] when `event_type` is a reasoning event.
///
/// Text-bearing reasoning deltas forward their `delta` payload; lifecycle
/// markers (part added, item added/done) forward an empty string, which still
/// produces a watchdog-refreshing activity tick without polluting the buffer.
/// Non-reasoning events are ignored.
pub(super) fn push_if_reasoning(
    event_type: Option<&str>,
    event: &Value,
    chunks: &mut Vec<StreamChunk>,
) {
    let Some(t) = event_type else { return };
    if !t.starts_with("response.reasoning") {
        return;
    }
    let delta = event
        .get("delta")
        .and_then(Value::as_str)
        .unwrap_or_default();
    chunks.push(StreamChunk::Thinking(delta.to_string()));
}

#[cfg(test)]
#[path = "codex_reasoning_tests.rs"]
mod tests;
