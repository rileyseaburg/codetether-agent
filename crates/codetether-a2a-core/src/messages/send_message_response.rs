use super::*;

/// The response union for `message/send` — either a full Task or a Message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
/// SendMessageResponse data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::SendMessageResponse::Message(codetether_a2a_core::Message { message_id: String::new(), role: codetether_a2a_core::MessageRole::User, parts: vec![], context_id: None, task_id: None, metadata: std::collections::HashMap::new(), extensions: vec![] });
/// match value {
///     codetether_a2a_core::SendMessageResponse::Task(_) => {},
///     codetether_a2a_core::SendMessageResponse::Message(_) => {},
/// }
/// ```
pub enum SendMessageResponse {
    Task(Task),
    Message(Message),
}
