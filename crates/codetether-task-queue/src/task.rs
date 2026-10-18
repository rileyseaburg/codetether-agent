//! Task payload shared by event ingestion and worker HTTP responses.

use serde::{Deserialize, Serialize};

/// Task accepted locally or received from Knative Eventing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnativeTask {
    /// Stable task identifier.
    pub task_id: String,
    /// Task title.
    pub title: String,
    /// Task instructions.
    pub description: String,
    /// Requested agent type.
    pub agent_type: String,
    #[serde(default)]
    /// Optional model selection.
    pub model: Option<String>,
    #[serde(default)]
    /// Optional task metadata.
    pub metadata: Option<serde_json::Value>,
    /// Scheduling priority value.
    pub priority: i32,
    /// Time the task was received.
    pub received_at: chrono::DateTime<chrono::Utc>,
    /// Current queue status string.
    pub status: String,
    #[serde(flatten, default)]
    /// Accepted completion receipt, when available.
    pub completion: super::TaskCompletion,
}
