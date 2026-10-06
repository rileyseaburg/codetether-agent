//! Task payload shared by event ingestion and worker HTTP responses.

use serde::{Deserialize, Serialize};

/// Task received from Knative Eventing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnativeTask {
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub agent_type: String,
    pub priority: i32,
    pub received_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}
