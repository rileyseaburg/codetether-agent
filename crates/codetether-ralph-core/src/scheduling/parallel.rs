//! Run dependency stages while retaining their shared isolation manager.
use super::*;
/// Schedule dependency stages in their supplied order.
///
/// # Arguments
/// `host` plans, checks limits and executes each complete stage.
/// # Returns
/// Completes after all allowed stages finish.
/// # Errors
/// Propagates stage execution or persistence errors.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &mut impl codetether_ralph_core::scheduling::ParallelHost) -> anyhow::Result<()> {
/// codetether_ralph_core::scheduling::run_parallel(host).await
/// # }
/// ```
pub async fn run_parallel(host: &mut impl ParallelHost) -> Result<()> {
    let stages = host.stages();
    let total_stages = stages.len();
    let worktree_mgr = host.manager();
    for (stage_idx, stage_stories) in stages.into_iter().enumerate() {
        if !host.begin(stage_idx, total_stages, &stage_stories) {
            break;
        }
        // Stories are already cloned from stages()
        let stories: Vec<UserStory> = stage_stories;

        host.execute_stage(stories, &worktree_mgr).await?;
    }

    Ok(())
}
