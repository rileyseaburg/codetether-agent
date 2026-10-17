use super::*;

/// Task status information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// TaskStatus data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::TaskStatus { state: codetether_a2a_core::TaskState::Submitted, message: None, timestamp: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct TaskStatus {
    pub state: TaskState,
    #[serde(default)]
    pub message: Option<Message>,
    pub timestamp: Option<String>,
}
