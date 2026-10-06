//! Task payload shared by event ingestion and worker HTTP responses.

use serde::{Deserialize, Serialize};

/// Task accepted locally or received from Knative Eventing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnativeTask {
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub agent_type: String,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    pub priority: i32,
    pub received_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    #[serde(flatten, default)]
    pub completion: super::TaskCompletion,
}
