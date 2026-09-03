//! Synchronization of worker roster state before a TUI draw.

use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(in crate::tui::app::event_loop) fn before_draw(
    app: &mut App,
    worker_bridge: &Option<TuiWorkerBridge>,
    synced_cursor: &mut Option<u64>,
) {
    if !should_sync(*synced_cursor, app.state.bus_cursor) {
        return;
    }
    *synced_cursor = Some(app.state.bus_cursor);
    crate::tui::app::worker_bridge::sync_worker_bridge_agents(app, worker_bridge);
}

pub(super) fn should_sync(synced: Option<u64>, current: u64) -> bool {
    synced != Some(current)
}
