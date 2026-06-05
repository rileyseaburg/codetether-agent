use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) fn attach(app: &mut App, worker_bridge: Option<&TuiWorkerBridge>) {
    if let Some(bridge) = worker_bridge {
        app.state
            .set_worker_bridge(bridge.worker_id.clone(), bridge.worker_name.clone());
        app.state.register_worker_agent("tui".to_string());
    }
}
