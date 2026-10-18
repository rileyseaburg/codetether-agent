//! Ordered progress, persistence and event delivery effects.
use codetether_ralph_core::{ProgressEntry, UserStory, events::RalphEvent};
/// Delivery bindings called at the original lifecycle positions.
/// # Implementors
/// Native hosts delegate to their existing event, store and bus services.
pub trait Reporting: Send + Sync {
    /// Send an event without blocking execution.
    fn try_send_event(&self, event: RalphEvent);
    /// Queue the story result for persistence.
    fn record_story_result(&self, story: &UserStory, passed: bool, error: Option<String>);
    /// Publish result learnings and the optional next story.
    fn bus_publish_story_result(
        &self,
        story: &UserStory,
        iteration: usize,
        learnings: &[String],
        next: Option<&str>,
    );
    /// Append the progress entry and full response.
    /// # Errors
    /// Returns progress-file errors.
    fn append_progress(&self, entry: &ProgressEntry, response: &str) -> anyhow::Result<()>;
}
