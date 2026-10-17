use super::*;

// ─── Streaming & Response Event Types ────────────────────────────────────

/// A task status update pushed via SSE / streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// TaskStatusUpdateEvent data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::TaskStatusUpdateEvent { id: String::new(), status: codetether_a2a_core::TaskStatus { state: codetether_a2a_core::TaskState::Submitted, message: None, timestamp: None }, is_final: false, metadata: std::collections::HashMap::new() };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct TaskStatusUpdateEvent {
    pub id: String,
    pub status: TaskStatus,
    #[serde(default, rename = "final")]
    pub is_final: bool,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}
