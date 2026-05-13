//! Tool-call session events.

#[path = "complete.rs"]
mod complete;
#[path = "start.rs"]
mod start;

pub(super) use complete::complete;
pub(super) use start::start;

use crate::session::SessionEvent;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn handle_event(
    app: &mut App,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) -> Option<SessionEvent> {
    match evt {
        SessionEvent::ToolCallStart { name, arguments } => {
            start(app, worker_bridge, name, arguments).await;
        }
        SessionEvent::ToolCallComplete {
            name,
            output,
            success,
            duration_ms,
        } => {
            complete(app, name, output, success, duration_ms);
        }
        other => return Some(other),
    }
    None
}
