//! Lifecycle session events.

use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::{handle_processing_started, handle_processing_stopped};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn handle_event(
    app: &mut App,
    session: &mut Session,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) -> Option<SessionEvent> {
    match evt {
        SessionEvent::Thinking => thinking(app, worker_bridge).await,
        SessionEvent::Done => done(app, worker_bridge).await,
        SessionEvent::SessionSync(updated) => sync(app, session, updated),
        other => return Some(other),
    }
    None
}

async fn thinking(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    handle_processing_started(app, worker_bridge).await;
    if app.state.processing_started_at.is_none() {
        app.state.begin_request_timing();
    }
    if app.state.streaming_start.is_none() {
        app.state.begin_streaming();
    }
    app.state.status = "Thinking…".to_string();
}

async fn done(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    handle_processing_stopped(app, worker_bridge).await;
    app.state.streaming_text.clear();
    app.state.complete_turn_timing();
    app.state.streaming_start = None;
    app.state.status = "Ready".to_string();
}

fn sync(app: &mut App, session: &mut Session, updated: Box<Session>) {
    *session = *updated;
    session.attach_global_bus_if_missing();
    app.state.session_id = Some(session.id.clone());
}
