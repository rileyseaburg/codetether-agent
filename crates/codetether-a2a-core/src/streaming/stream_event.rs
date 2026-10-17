use super::*;

/// A single frame on a streaming response (SSE `data:` payload).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
/// StreamEvent data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::StreamEvent::StatusUpdate(codetether_a2a_core::TaskStatusUpdateEvent { id: String::new(), status: codetether_a2a_core::TaskStatus { state: codetether_a2a_core::TaskState::Submitted, message: None, timestamp: None }, is_final: false, metadata: std::collections::HashMap::new() });
/// match value {
///     codetether_a2a_core::StreamEvent::StatusUpdate(_) => {},
///     codetether_a2a_core::StreamEvent::ArtifactUpdate(_) => {},
/// }
/// ```
pub enum StreamEvent {
    StatusUpdate(TaskStatusUpdateEvent),
    ArtifactUpdate(TaskArtifactUpdateEvent),
}
