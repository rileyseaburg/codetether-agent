//! Shared relay context data.
use super::*;

/// Context additions produced by one relay turn.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::ContextDelta;
/// let delta = ContextDelta::default();
/// assert!(delta.facts.is_empty());
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContextDelta {
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
