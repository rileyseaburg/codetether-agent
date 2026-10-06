//! Dispatch request and receipt contracts.

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct DispatchTaskRequest {
    pub title: String,
    pub description: String,
    pub agent_type: Option<String>,
    pub model: Option<String>,
    pub priority: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

/// A receipt for process-local acceptance, never proof of execution.
#[derive(Debug, Serialize)]
pub(crate) struct DispatchTaskResponse {
    pub task_id: String,
    pub status: &'static str,
    pub dispatched_via_knative: bool,
    pub dispatch_mode: &'static str,
    pub durable: bool,
}
