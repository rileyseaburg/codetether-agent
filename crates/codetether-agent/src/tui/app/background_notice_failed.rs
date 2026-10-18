//! Apply failed session runtime notices.

use crate::session::Session;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "background_notice_failed/host.rs"]
mod host;

pub(super) async fn apply(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    session: Session,
    error: String,
) {
    codetether_session_events::completion::failure::apply(
        &mut host::Native {
            app,
            slot,
            worker_bridge,
        },
        session,
        error,
    )
    .await;
}
