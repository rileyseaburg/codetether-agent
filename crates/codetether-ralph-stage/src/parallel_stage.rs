//! Stop completed/exhausted runs before announcing the next stage.
use super::*;
/// Check completion and iteration limits before announcing a stage.
/// # Arguments
/// The live state, stage index, total stage count and ready stories.
/// # Returns
/// True when this stage may execute; false when the run should stop.
/// # Examples
/// ```rust,no_run
/// # fn example(host: &mut impl codetether_ralph_stage::Host) {
/// let _ = codetether_ralph_stage::begin_parallel_stage(host, 0, 1, &[]);
/// # }
/// ```
pub fn begin_parallel_stage<H: Host>(
    host: &mut H,
    stage_idx: usize,
    total_stages: usize,
    stage_stories: &[UserStory],
) -> bool {
    if host.inputs().state.prd.is_complete() {
        info!(target: "codetether_agent::ralph::ralph_loop::parallel_stage", "All stories complete!");
        host.state_mut().status = RalphStatus::Completed;
        return false;
    }

    if host.inputs().state.current_iteration >= host.inputs().state.max_iterations {
        return false;
    }

    let story_count = stage_stories.len();
    info!(target: "codetether_agent::ralph::ralph_loop::parallel_stage",
        "=== Stage {}/{}: {} stories in parallel ===",
        stage_idx + 1,
        total_stages,
        story_count
    );

    true
}
