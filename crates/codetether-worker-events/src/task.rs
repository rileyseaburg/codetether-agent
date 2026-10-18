//! Incoming task fields decoded from worker SSE events.
use serde::{Deserialize, Serialize};

/// Incoming task from the A2A server via SSE.
/// # Examples
/// ```
/// let task = codetether_worker_events::IncomingTask { task_id: "task-1".into(), message: "hello".into(), from_agent: None };
/// assert_eq!(task.task_id, "task-1");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingTask {
    /// Server task identifier.
    pub task_id: String,
    /// Task input text.
    pub message: String,
    /// Optional originating agent name.
    pub from_agent: Option<String>,
}
