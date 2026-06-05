//! Apply session runtime notices to TUI state.

use std::path::Path;

use crate::session::Session;
use crate::tui::app::session_runtime::{SessionNotice, SessionSlot};
use crate::tui::app::worker_bridge::handle_processing_stopped;
use crate::tui::app::{session_sync::refresh_sessions, state::App};
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "background_notice_failed.rs"]
mod failed_notice;

pub(super) async fn apply(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    notice: SessionNotice,
) {
    match notice {
        SessionNotice::Started => {}
        SessionNotice::Finished(session) => finished(app, cwd, slot, worker_bridge, session).await,
        SessionNotice::Failed { session, error } => {
            failed_notice::apply(app, slot, worker_bridge, session, error).await;
        }
    }
}

async fn finished(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    session: Session,
) {
    if session.id != slot.view().id() {
        tracing::warn!(stale_id = %session.id, current_id = %slot.view().id(), "Discarding stale session result");
        let _ = session.save().await;
        refresh_sessions(app, cwd).await;
        return;
    }
    if app.state.processing {
        handle_processing_stopped(app, worker_bridge).await;
        app.state.clear_request_timing();
    }
    slot.restore(session);
    let Some(session) = slot.borrow_mut() else {
        return;
    };
    session.attach_global_bus_if_missing();
    crate::tui::app::turn_cancel::clear(app);
    app.state.session_id = Some(session.id.clone());
    let _ = session.save().await;
    refresh_sessions(app, cwd).await;
}
