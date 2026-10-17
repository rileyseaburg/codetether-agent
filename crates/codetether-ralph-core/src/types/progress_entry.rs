//! ProgressEntry serializable contract.
use super::*;

/// A progress log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let entry = codetether_ralph_core::ProgressEntry { story_id: "US-1".into(), iteration: 0, status: "pending".into(), learnings: vec![], files_changed: vec![], timestamp: String::new() };
/// assert_eq!(entry.iteration, 0);
/// ```
pub struct ProgressEntry {
    /// Story ID being worked on
    pub story_id: String,

    /// Iteration number
    pub iteration: usize,

    /// Status of this attempt
    pub status: String,

    /// What was learned
    #[serde(default)]
    pub learnings: Vec<String>,

    /// Files changed
    #[serde(default)]
    pub files_changed: Vec<String>,

    /// Timestamp
    pub timestamp: String,
}
