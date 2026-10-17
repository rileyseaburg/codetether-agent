use super::*;

/// An additional transport interface for an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// AgentInterface data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::AgentInterface { transport: String::new(), url: String::new(), content_types: vec![] };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct AgentInterface {
    /// Transport name (e.g. "GRPC", "JSONRPC", "WEBSOCKET")
    pub transport: String,
    /// Endpoint URL for this interface
    pub url: String,
    /// Content-types accepted at this interface
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content_types: Vec<String>,
}
