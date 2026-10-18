//! Native projections for shared worker bridge synchronization.
use crate::tui::app::state::App;
use crate::tui::worker_bridge::{TuiWorkerBridge, WorkerBridgeCmd};
use codetether_tui_worker::synchronization;

pub async fn send_worker_bridge_cmd(
    worker_bridge: &Option<TuiWorkerBridge>,
    cmd: WorkerBridgeCmd,
) -> bool {
    synchronization::send(worker_bridge.as_ref().map(|bridge| &bridge.cmd_tx), cmd)
}

pub fn sync_worker_bridge_agents(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    synchronization::sync_agents(
        &mut app.state.worker,
        worker_bridge.as_ref().map(|bridge| &bridge.cmd_tx),
        app.state
            .bus_log
            .entries
            .iter()
            .map(|message| (message.kind.as_str(), message.summary.as_str())),
    );
}

pub async fn handle_processing_started(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    synchronization::set_processing(
        &mut app.state.worker,
        &mut app.state.processing,
        worker_bridge.as_ref().map(|bridge| &bridge.cmd_tx),
        true,
    );
}

pub async fn handle_processing_stopped(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    synchronization::set_processing(
        &mut app.state.worker,
        &mut app.state.processing,
        worker_bridge.as_ref().map(|bridge| &bridge.cmd_tx),
        false,
    );
}
