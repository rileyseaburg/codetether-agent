//! StoryResultEntry persistence contract.
use super::*;

/// Result of a single story execution
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let entry = codetether_ralph_core::state_store::StoryResultEntry { story_id: "US-1".into(), title: "Example".into(), passed: false, iteration: 0, error: None };
/// assert!(!entry.passed);
/// ```
pub struct StoryResultEntry {
    /// Story identifier.
    pub story_id: String,
    /// Human-readable story title.
    pub title: String,
    /// Whether this story passed.
    pub passed: bool,
    /// Iteration that produced the result.
    pub iteration: usize,
    /// Most recent error, when present.
    pub error: Option<String>,
}
