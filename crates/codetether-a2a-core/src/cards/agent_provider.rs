use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// AgentProvider data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::AgentProvider { organization: String::new(), url: String::new() };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct AgentProvider {
    pub organization: String,
    pub url: String,
}
