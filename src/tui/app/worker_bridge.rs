use crate::tui::app::state::App;
use crate::tui::worker_bridge::{TuiWorkerBridge, WorkerBridgeCmd};

pub async fn send_worker_bridge_cmd(
    worker_bridge: &Option<TuiWorkerBridge>,
    cmd: WorkerBridgeCmd,
) -> bool {
    match worker_bridge {
        Some(bridge) => bridge.cmd_tx.send(cmd).await.is_ok(),
        None => false,
    }
}

pub fn sync_worker_bridge_agents(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    let Some(bridge) = worker_bridge.as_ref() else {
        return;
    };

    let mut desired = std::collections::HashSet::new();
    desired.insert("tui".to_string());

    for message in &app.state.bus_log.entries {
        if message.kind == "READY"
            && let Some(agent) = message.summary.split_whitespace().next()
        {
            desired.insert(agent.to_string());
        }
    }

    let current = app.state.worker_bridge_registered_agents.clone();
    for name in desired.difference(&current) {
        let _ = bridge.cmd_tx.try_send(WorkerBridgeCmd::RegisterAgent {
            name: name.clone(),
            instructions: format!("Protocol-observed A2A agent {name}"),
        });
    }
    for name in current.difference(&desired) {
        let _ = bridge
            .cmd_tx
            .try_send(WorkerBridgeCmd::DeregisterAgent { name: name.clone() });
    }
    app.state.worker_bridge_registered_agents = desired;
}

pub async fn handle_processing_started(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    app.state.processing = true;
    app.state.sync_worker_bridge_processing(true);
    let _ = send_worker_bridge_cmd(worker_bridge, WorkerBridgeCmd::SetProcessing(true)).await;
}

pub async fn handle_processing_stopped(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    app.state.processing = false;
    app.state.sync_worker_bridge_processing(false);
    let _ = send_worker_bridge_cmd(worker_bridge, WorkerBridgeCmd::SetProcessing(false)).await;
}
