//! Live readable reasoning previews and lifecycle cleanup.
use crate::session::SessionEvent;
use crate::tui::{
    app::{session_runtime::SessionSlot, state::App},
    worker_bridge::TuiWorkerBridge,
};

pub(super) async fn handle_event(
    app: &mut App,
    slot: &mut SessionSlot,
    bridge: &Option<TuiWorkerBridge>,
    event: SessionEvent,
) -> Option<SessionEvent> {
    let event = match event {
        SessionEvent::ThinkingChunk(text) => {
            let text = codetether_util::truncate_bytes_safe(
                &text,
                crate::tui::constants::MAX_STREAMING_TEXT_BYTES,
            )
            .to_string();
            app.state.streaming.streaming_reasoning = text;
            app.state.message_cache.invalidate_streaming();
            SessionEvent::Thinking
        }
        event @ (SessionEvent::ThinkingComplete(_)
        | SessionEvent::TextChunk(_)
        | SessionEvent::TextComplete(_)
        | SessionEvent::ToolCallStart { .. }
        | SessionEvent::Done
        | SessionEvent::Error(_)
        | SessionEvent::StreamRetry(_)
        | SessionEvent::SessionSync(_)) => {
            app.state.streaming.streaming_reasoning.clear();
            app.state.message_cache.invalidate_streaming();
            event
        }
        event => event,
    };
    super::lifecycle::handle_event(app, slot, bridge, event).await
}
