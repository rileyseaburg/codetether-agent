//! Project native worker status into the protocol summary.
use crate::tui::{
    app::state::App,
    bus_log::{ProtocolSummary, render_bus_log_with_summary},
};
use ratatui::Frame;
pub(super) fn render(f: &mut Frame, app: &mut App) {
    let worker = &app.state.worker;
    let mut registered_agents = worker
        .worker_bridge_registered_agents
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    registered_agents.sort();
    let summary = ProtocolSummary {
        cwd_display: app.state.cwd_display.clone(),
        worker_id: worker.worker_id.clone(),
        worker_name: worker.worker_name.clone(),
        a2a_connected: worker.a2a_connected,
        processing: worker.worker_bridge_processing_state,
        registered_agents,
        queued_tasks: worker.worker_task_queue.len(),
        recent_task: worker.recent_tasks.last().cloned(),
        peer_endpoint_ready: app.state.peer_endpoint_ready,
    };
    render_bus_log_with_summary(f, &mut app.state.bus_log, f.area(), Some(summary))
}
