//! Map session events to the existing observer JSON payloads.
use super::*;
use codetether_a2a_core::observer_payloads as payload;
pub(super) fn data<S>(event: &SessionEvent<S>) -> Option<serde_json::Value> {
    Some(match event {
        SessionEvent::Thinking
        | SessionEvent::ToolCallStart { .. }
        | SessionEvent::ToolCallComplete { .. }
        | SessionEvent::ToolHeartbeat { .. }
        | SessionEvent::ToolOutputChunk { .. } => server_session_event::data(&event),
        SessionEvent::TextChunk(text) => payload::text_chunk(text),
        SessionEvent::TextComplete(text) => payload::text_complete(text),
        SessionEvent::ThinkingComplete(thought) => payload::thinking_complete(thought),
        SessionEvent::UsageReport {
            prompt_tokens,
            completion_tokens,
            duration_ms,
            model,
        } => payload::usage_report(*prompt_tokens, *completion_tokens, *duration_ms, model),
        SessionEvent::Done => payload::done(),
        SessionEvent::Error(err) => payload::error(err),
        SessionEvent::SessionSync(_) => {
            return None; // Don't emit session sync to SSE
        }
        // New non-exhaustive variants (context management, RLM
        // progress, token estimates). They are handled by the
        // dedicated SessionBus subscribers rather than SSE.
        _ => return None,
    })
}
