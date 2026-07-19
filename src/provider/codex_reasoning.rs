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

#[path = "codex_reasoning/signature.rs"]
mod signature;

use crate::provider::{ContentPart, StreamChunk};
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

pub(super) fn push_item_if_reasoning(item: &Value, chunks: &mut Vec<StreamChunk>) -> bool {
    if item.get("type").and_then(Value::as_str) != Some("reasoning") {
        return false;
    }
    let chunk = signature::encode(item).unwrap_or_default();
    chunks.push(StreamChunk::Thinking(chunk));
    true
}

pub(super) fn checkpoint(item: &Value) -> Option<ContentPart> {
    let signature = signature::encode(item)?;
    Some(ContentPart::Thinking {
        text: String::new(),
        signature: Some(signature),
    })
}

pub(crate) fn is_signature(value: &str) -> bool {
    signature::decode(value).is_some()
}

pub(super) fn decode_signature(value: &str) -> Option<Value> {
    signature::decode(value)
}

#[cfg(test)]
#[path = "codex_reasoning_tests.rs"]
mod tests;
