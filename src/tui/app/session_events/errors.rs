//! Error session events.

#[path = "smart_switch.rs"]
mod smart_switch;

use crate::session::SessionEvent;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::handle_processing_stopped;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn handle_event(
    app: &mut App,
    slot: &SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) -> Option<SessionEvent> {
    match evt {
        SessionEvent::Error(err) => handle(app, slot, worker_bridge, err).await,
        other => return Some(other),
    }
    None
}

async fn handle(
    app: &mut App,
    slot: &SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    err: String,
) {
    handle_processing_stopped(app, worker_bridge).await;
    app.state.clear_streaming_text();
    app.state.complete_request_timing();
    smart_switch::schedule(app, slot, &err);
    app.state
        .messages
        .push(ChatMessage::new(MessageType::Error, err));
    app.state.status = "Error".to_string();
    if app.state.chat_auto_follow {
        app.state.scroll_to_bottom();
    }
}
