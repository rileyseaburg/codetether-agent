use super::*;

/// A task artifact update pushed via SSE / streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// TaskArtifactUpdateEvent data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::TaskArtifactUpdateEvent { id: String::new(), artifact: codetether_a2a_core::Artifact { artifact_id: String::new(), parts: vec![], name: None, description: None, metadata: std::collections::HashMap::new(), extensions: vec![] }, metadata: std::collections::HashMap::new() };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct TaskArtifactUpdateEvent {
    pub id: String,
    pub artifact: Artifact,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}
