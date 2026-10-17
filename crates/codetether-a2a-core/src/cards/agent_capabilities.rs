use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// AgentCapabilities data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::AgentCapabilities { streaming: false, push_notifications: false, state_transition_history: false, extensions: vec![] };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct AgentCapabilities {
    #[serde(default)]
    pub streaming: bool,
    #[serde(default)]
    pub push_notifications: bool,
    #[serde(default)]
    pub state_transition_history: bool,
    /// Extensions supported by this agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<AgentExtension>,
}
