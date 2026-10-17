use super::*;

// ─── Agent Card Extension Types ──────────────────────────────────────────

/// An agent-level extension declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// AgentExtension data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::AgentExtension { uri: String::new(), description: None, required: false, params: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct AgentExtension {
    /// URI identifying the extension
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this extension is required to interact with the agent
    #[serde(default)]
    pub required: bool,
    /// Extension-specific configuration parameters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}
