//! HTTP task-claim response fields.
use serde::{Deserialize, Serialize};
/// Task-claim result shared by the control plane and workers.

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskClaimResponse {
    pub task_id: String,
    pub worker_id: String,
    pub run_id: Option<String>,
    pub attempt_id: Option<String>,
    pub tenant_id: Option<String>,
    pub user_id: Option<String>,
    pub agent_identity_id: Option<String>,
    #[serde(default)]
    pub task_timeout_seconds: Option<u64>,
    #[serde(default)]
    pub provider_keys: Option<serde_json::Value>,
    #[serde(default)]
    pub provider_key_source: Option<String>,
}
