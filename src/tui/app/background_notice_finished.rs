//! Apply a successfully finished session notice.
//!
//! The session list is updated in memory rather than rescanned from disk:
//! a full [`refresh_sessions`](crate::tui::app::session_sync::refresh_sessions) walks thousands of files and blocked the UI
//! task for seconds after every reply on large workspaces.

use std::path::Path;

use crate::session::Session;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "background_notice_finished/host/mod.rs"]
mod host;

pub(super) async fn apply(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    session: Session,
) {
    codetether_session_events::completion::success::apply(
        &mut host::Native {
            app,
            cwd,
            slot,
            worker_bridge,
        },
        session,
    )
    .await;
}
