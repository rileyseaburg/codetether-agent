//! HTTP payload and status serialization contracts.
use super::*;

/// Publish a message to the agent bus.
#[derive(Deserialize)]
pub(super) struct BusPublishRequest {
    pub(super) topic: String,
    pub(super) payload: serde_json::Value,
}

/// Response types
#[derive(Serialize)]
pub(super) struct WorkerStatusResponse {
    pub(super) connected: bool,
    pub(super) worker_id: Option<String>,
    pub(super) heartbeat: Option<HeartbeatInfo>,
}

#[derive(Serialize)]
pub(super) struct HeartbeatInfo {
    pub(super) status: String,
    pub(super) active_tasks: usize,
    pub(super) agent_name: String,
}
