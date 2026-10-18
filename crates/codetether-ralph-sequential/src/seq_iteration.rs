//! Sequential Ralph iteration handling.
use crate::*;
use codetether_ralph_core::events::RalphEvent;
/// Persist and announce the next iteration, then select a ready story.
///
/// # Arguments
/// Receives the live native host and the current story or response when applicable.
/// # Returns
/// The next ready story, or None when complete or blocked by dependencies.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &mut impl codetether_ralph_sequential::Host) -> anyhow::Result<()> {
/// codetether_ralph_sequential::begin_sequential_iteration(host);
/// # Ok(())
/// # }
/// ```
pub fn begin_sequential_iteration(host: &mut impl Host) -> Option<UserStory> {
    host.state_mut().current_iteration += 1;

    // Persist iteration update
    codetether_ralph_core::state_store::updates::iteration(
        host.store(),
        host.run_id(),
        host.state(),
    );

    info!(target: "codetether_agent::ralph::ralph_loop::seq_iteration",
        "=== Ralph iteration {} of {} ===",
        host.state().current_iteration, host.state().max_iterations
    );

    // Emit iteration event
    host.try_send_event(RalphEvent::IterationStarted {
        iteration: host.state().current_iteration,
        max_iterations: host.state().max_iterations,
    });

    // Check if all stories are complete
    if host.state().prd.is_complete() {
        info!(target: "codetether_agent::ralph::ralph_loop::seq_iteration", "All stories complete!");
        host.state_mut().status = RalphStatus::Completed;
        return None;
    }

    // Get next story to work on
    let story = match host.state().prd.next_story() {
        Some(s) => s.clone(),
        None => {
            warn!(target: "codetether_agent::ralph::ralph_loop::seq_iteration", "No available stories (dependencies not met)");
            return None;
        }
    };

    info!(target: "codetether_agent::ralph::ralph_loop::seq_iteration", "Working on story: {} - {}", story.id, story.title);

    // Emit StoryStarted event
    host.try_send_event(RalphEvent::StoryStarted {
        story_id: story.id.clone(),
    });

    Some(story)
}
