//! Apply session runtime notices to TUI state.

use std::path::Path;

use crate::tui::app::session_runtime::{SessionNotice, SessionSlot};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "background_notice_failed.rs"]
mod failed_notice;
#[path = "background_notice_finished.rs"]
mod finished_notice;

pub(super) async fn apply(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    notice: SessionNotice,
) {
    match notice {
        SessionNotice::Started => {}
        SessionNotice::Finished(session) => {
            finished_notice::apply(app, cwd, slot, worker_bridge, session).await;
        }
        SessionNotice::Failed { session, error } => {
            failed_notice::apply(app, slot, worker_bridge, session, error).await;
        }
    }
}
