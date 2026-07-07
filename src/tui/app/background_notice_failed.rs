//! Apply failed session runtime notices.

use crate::session::Session;
use crate::tui::app::background::stream_reconnect;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::handle_processing_stopped;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::super::runtime_retry;

pub(super) async fn apply(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    session: Session,
    error: String,
) {
    slot.restore(session);
    handle_processing_stopped(app, worker_bridge).await;
    app.state.complete_request_timing();
    // Schedule a same-model reconnect for transient stream disconnects BEFORE
    // smart-switch so a broken-pipe doesn't needlessly change providers.
    stream_reconnect::schedule(app, &error);
    runtime_retry::schedule(app, slot, &error);
    app.state
        .messages
        .push(ChatMessage::new(MessageType::Error, error));
    app.state.status = "Request failed".to_string();
    app.state.scroll_to_bottom();
}
