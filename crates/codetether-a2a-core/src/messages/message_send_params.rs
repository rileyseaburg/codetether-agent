use super::*;

/// Message send parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// MessageSendParams data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::MessageSendParams { message: codetether_a2a_core::Message { message_id: String::new(), role: codetether_a2a_core::MessageRole::User, parts: vec![], context_id: None, task_id: None, metadata: std::collections::HashMap::new(), extensions: vec![] }, configuration: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct MessageSendParams {
    pub message: Message,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<MessageSendConfiguration>,
}
