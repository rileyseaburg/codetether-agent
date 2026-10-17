//! Failed progress records for model-call errors.
use crate::{ProgressEntry, UserStory};
/// Construct a failure record and timestamp it.
///
/// # Arguments
/// `story` identifies work, `iteration` its attempt, and `error` the failure.
/// # Returns
/// A failed record with empty learnings and changed files.
/// # Examples
/// ```rust,no_run
/// fn example(story: &codetether_ralph_core::UserStory) {
/// let entry = codetether_ralph_core::progress::failed_entry(story, 1, &anyhow::anyhow!("offline"));
/// assert_eq!(entry.status, "failed: offline");
/// }
/// ```
pub fn failed_entry(story: &UserStory, iteration: usize, e: &anyhow::Error) -> ProgressEntry {
    ProgressEntry {
        story_id: story.id.clone(),
        iteration,
        status: format!("failed: {}", e),
        learnings: Vec::new(),
        files_changed: Vec::new(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}
