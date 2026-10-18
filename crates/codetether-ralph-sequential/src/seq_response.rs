//! Sequential Ralph response handling.
use crate::*;
/// Record model output and apply quality and verification completion gates.
///
/// # Arguments
/// Receives the live native host and the current story or response when applicable.
/// # Returns
/// Completion after all ordered response effects succeed.
/// # Errors
/// Propagates progress, quality, commit and PRD save failures.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &mut impl codetether_ralph_sequential::Host, story: &codetether_ralph_core::UserStory) -> anyhow::Result<()> {
/// codetether_ralph_sequential::handle_sequential_response(host, story, "response").await?;
/// # Ok(())
/// # }
/// ```
pub async fn handle_sequential_response(
    host: &mut impl Host,
    story: &UserStory,
    response: &str,
) -> anyhow::Result<()> {
    // Log progress
    let entry = codetether_ralph_core::progress::completed_entry(
        story,
        host.state().current_iteration,
        response,
        |text| host.extract_learnings(text),
    );
    host.append_progress(&entry, response)?;
    let entry_learnings = entry.learnings.clone();
    host.state_mut().progress_log.push(entry);

    if host.config().quality_checks_enabled {
        if host.run_quality_gates_with_events(&story.id).await? {
            finish_verified_story(host, story, &entry_learnings).await?;
        } else {
            record_quality_failure(host, story, &entry_learnings);
        }
    } else {
        finish_unchecked_story(host, story).await?;
    }
    Ok(())
}
