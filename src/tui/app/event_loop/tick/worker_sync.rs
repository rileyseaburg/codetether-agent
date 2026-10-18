//! Native roster synchronization under shared bus-cursor policy.
use crate::tui::{app::state::App, worker_bridge::TuiWorkerBridge};
pub(in crate::tui::app::event_loop) fn before_draw(
    app: &mut App,
    worker_bridge: &Option<TuiWorkerBridge>,
    synced_cursor: &mut Option<u64>,
) {
    let current = app.state.bus_cursor;
    codetether_tui_tick::worker_sync::before_draw(synced_cursor, current, || {
        crate::tui::app::worker_bridge::sync_worker_bridge_agents(app, worker_bridge);
    });
}
