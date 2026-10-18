//! Apply bridge commands to the shared heartbeat state.
use crate::WorkerBridgeCmd;
use codetether_a2a_worker_core::{HeartbeatState, WorkerStatus};
pub(crate) async fn apply(cmd: WorkerBridgeCmd, heartbeat_state: &HeartbeatState) -> bool {
    match cmd {
        WorkerBridgeCmd::RegisterAgent { name, instructions } => {
            tracing::info!(agent = %name, "Registering sub-agent with A2A server");
            tracing::debug!(
                agent = %name,
                instructions_len = instructions.len(),
                "Tracking sub-agent in heartbeat state"
            );
            heartbeat_state.register_sub_agent(name).await;
        }
        WorkerBridgeCmd::DeregisterAgent { name } => {
            tracing::info!(agent = %name, "Deregistering sub-agent from A2A server");
            heartbeat_state.deregister_sub_agent(&name).await;
        }
        WorkerBridgeCmd::SetProcessing(processing) => {
            let status = if processing {
                WorkerStatus::Processing
            } else {
                WorkerStatus::Idle
            };
            heartbeat_state.set_status(status).await;
        }
        WorkerBridgeCmd::Shutdown => return true,
    }
    false
}
