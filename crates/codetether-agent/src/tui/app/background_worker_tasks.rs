//! Worker-task queue helpers for the background update loop.

use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) fn queue_worker_tasks(app: &mut App, worker_bridge: &mut Option<TuiWorkerBridge>) {
    if let Some(bridge) = worker_bridge.as_mut() {
        codetether_tui_worker::tasks::ingest(
            &mut app.state.worker,
            &mut app.state.status,
            &mut bridge.task_rx,
        );
    }
}
