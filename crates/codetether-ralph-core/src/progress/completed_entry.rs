//! Completed progress records with caller-selected learning extraction.
use crate::{ProgressEntry, UserStory};
/// Construct a completed record, extracting learnings before timestamping.
///
/// # Arguments
/// `story` identifies the work; `iteration` identifies its attempt; `response`
/// supplies the text and `extract` supplies its learning-extraction policy.
/// # Returns
/// A completed record with no inferred changed files and a current timestamp.
/// # Examples
/// ```rust,no_run
/// fn example(story: &codetether_ralph_core::UserStory) {
/// let entry = codetether_ralph_core::progress::completed_entry(story, 1, "learned X",
///     codetether_ralph_core::presentation::extract_learnings);
/// assert_eq!(entry.learnings, vec!["learned X"]);
/// }
/// ```
pub fn completed_entry(
    story: &UserStory,
    iteration: usize,
    response: &str,
    extract: impl FnOnce(&str) -> Vec<String>,
) -> ProgressEntry {
    ProgressEntry {
        story_id: story.id.clone(),
        iteration,
        status: "completed".to_string(),
        learnings: extract(response),
        files_changed: Vec::new(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}
