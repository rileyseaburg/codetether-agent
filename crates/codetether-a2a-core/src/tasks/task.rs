use super::*;

/// An A2A Task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Task data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::Task { id: String::new(), context_id: None, status: codetether_a2a_core::TaskStatus { state: codetether_a2a_core::TaskState::Submitted, message: None, timestamp: None }, artifacts: vec![], history: vec![], metadata: std::collections::HashMap::new() };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct Task {
    pub id: String,
    pub context_id: Option<String>,
    pub status: TaskStatus,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
    #[serde(default)]
    pub history: Vec<Message>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}
