//! Sequential Ralph error handling.
use crate::*;
use codetether_ralph_core::events::RalphEvent;
/// Record a model error in progress and emit its story event.
///
/// # Arguments
/// Receives the live native host and the current story or response when applicable.
/// # Returns
/// Returns after recording the failure.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &impl codetether_ralph_sequential::Host, story: &codetether_ralph_core::UserStory) -> anyhow::Result<()> {
/// codetether_ralph_sequential::record_sequential_error(host, story, &anyhow::anyhow!("model error"));
/// # Ok(())
/// # }
/// ```
pub fn record_sequential_error(host: &mut impl Host, story: &UserStory, e: &anyhow::Error) {
    warn!(target: "codetether_agent::ralph::ralph_loop::seq_error", "LLM call failed: {}", e);
    let entry =
        codetether_ralph_core::progress::failed_entry(story, host.state().current_iteration, e);
    host.state_mut().progress_log.push(entry);

    host.try_send_event(RalphEvent::StoryError {
        story_id: story.id.clone(),
        error: format!("{}", e),
    });
}
