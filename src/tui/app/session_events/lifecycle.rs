//! Lifecycle session events.

use crate::session::{Session, SessionEvent};
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::{handle_processing_started, handle_processing_stopped};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn handle_event(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) -> Option<SessionEvent> {
    match evt {
        SessionEvent::Thinking => thinking(app, worker_bridge).await,
        SessionEvent::Done => done(app, worker_bridge).await,
        SessionEvent::SessionSync(updated) => sync(app, slot, updated),
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
    app.state.clear_streaming_text();
    app.state.complete_turn_timing();
    app.state.streaming_start = None;
    // NOTE: do NOT reset main_watchdog_restart_count here. The count must
    // survive a completed turn so a pathological "completes-then-immediately-
    // stalls" storm still climbs to WATCHDOG_MAX_RESTARTS instead of being
    // zeroed on every completion. The count is reset only when a genuine new
    // user prompt is dispatched (see dispatch_prompt).
    // Clear any watchdog notice now that a turn has completed; otherwise the
    // retry guard (notification.is_some() && !processing) keeps resubmitting the
    // same prompt forever.
    app.state.watchdog_notification = None;
    app.state.main_inflight_prompt = None;
    app.state.status = "Ready".to_string();
}

fn sync(app: &mut App, slot: &mut SessionSlot, updated: Box<Session>) {
    slot.restore(*updated);
    if let Some(session) = slot.borrow_mut() {
        session.attach_global_bus_if_missing();
        app.state.session_id = Some(session.id.clone());
    }
}
