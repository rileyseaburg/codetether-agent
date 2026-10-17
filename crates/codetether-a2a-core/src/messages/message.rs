use super::*;

/// An A2A Message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Message data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::Message { message_id: String::new(), role: codetether_a2a_core::MessageRole::User, parts: vec![], context_id: None, task_id: None, metadata: std::collections::HashMap::new(), extensions: vec![] };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct Message {
    pub message_id: String,
    pub role: MessageRole,
    pub parts: Vec<Part>,
    #[serde(default)]
    pub context_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// URIs of extensions present in or contributing to this Message
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<String>,
}
