//! Map session events to the existing observer JSON payloads.
use super::*;
pub(super) fn data(event: &SessionEvent) -> Option<serde_json::Value> {
    Some(match event {
        SessionEvent::Thinking
        | SessionEvent::ToolCallStart { .. }
        | SessionEvent::ToolCallComplete { .. }
        | SessionEvent::ToolHeartbeat { .. }
        | SessionEvent::ToolOutputChunk { .. } => server_session_event::data(&event),
        SessionEvent::TextChunk(text) => {
            serde_json::json!({ "type": "text_chunk", "text": text })
        }
        SessionEvent::TextComplete(text) => {
            serde_json::json!({ "type": "text_complete", "text": text })
        }
        SessionEvent::ThinkingComplete(thought) => {
            serde_json::json!({ "type": "thinking_complete", "thought": thought })
        }
        SessionEvent::UsageReport {
            prompt_tokens,
            completion_tokens,
            duration_ms,
            model,
        } => {
            serde_json::json!({
                "type": "usage_report",
                "prompt_tokens": prompt_tokens,
                "completion_tokens": completion_tokens,
                "duration_ms": duration_ms,
                "model": model
            })
        }
        SessionEvent::Done => {
            serde_json::json!({ "type": "done" })
        }
        SessionEvent::Error(err) => {
            serde_json::json!({ "type": "error", "error": err })
        }
        SessionEvent::SessionSync(_) => {
            return None; // Don't emit session sync to SSE
        }
        // New non-exhaustive variants (context management, RLM
        // progress, token estimates). They are handled by the
        // dedicated SessionBus subscribers rather than SSE.
        _ => return None,
    })
}
