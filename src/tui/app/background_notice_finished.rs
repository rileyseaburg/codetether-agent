//! Apply a successfully finished session notice.
//!
//! The session list is updated in memory rather than rescanned from disk:
//! a full [`refresh_sessions`] walks thousands of files and blocked the UI
//! task for seconds after every reply on large workspaces.

use std::path::Path;

use crate::session::Session;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::session_sync::{refresh_sessions, upsert_active_session};
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::handle_processing_stopped;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn apply(
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
    crate::tui::app::background::stream_reconnect_execute::on_success(app);
    app.state.session_id = Some(session.id.clone());
    let _ = session.save().await;
    upsert_active_session(app, session);
}
