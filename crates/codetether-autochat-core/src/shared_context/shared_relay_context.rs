//! Shared relay context data.
use super::*;

/// Bounded shared facts, decisions, risks, actions and evidence.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::SharedRelayContext;
/// let context = SharedRelayContext::default();
/// assert!(context.is_empty());
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SharedRelayContext {
    #[serde(default)]
    pub facts: Vec<String>,
    #[serde(default)]
    pub decisions: Vec<String>,
    #[serde(default)]
    pub risks: Vec<String>,
    #[serde(default)]
    pub next_actions: Vec<String>,
    #[serde(default)]
    pub evidence: Vec<String>,
}
