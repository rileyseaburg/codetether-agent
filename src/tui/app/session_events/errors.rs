//! Error session events.

#[path = "smart_switch.rs"]
mod smart_switch;

use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::handle_processing_stopped;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn handle_event(
    app: &mut App,
    session: &Session,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) -> Option<SessionEvent> {
    match evt {
        SessionEvent::Error(err) => handle(app, session, worker_bridge, err).await,
        other => return Some(other),
    }
    None
}

async fn handle(
    app: &mut App,
    session: &Session,
    worker_bridge: &Option<TuiWorkerBridge>,
    err: String,
) {
    handle_processing_stopped(app, worker_bridge).await;
    app.state.streaming_text.clear();
    app.state.complete_request_timing();
    smart_switch::schedule(app, session, &err);
    app.state
        .messages
        .push(ChatMessage::new(MessageType::Error, err));
    app.state.status = "Error".to_string();
    if app.state.chat_auto_follow {
        app.state.scroll_to_bottom();
    }
}
