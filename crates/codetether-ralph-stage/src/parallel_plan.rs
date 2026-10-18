//! Plan dependency stages and initialize optional worktree isolation.
use super::*;
/// Snapshot the dependency-ordered story groups and log the stage plan.
/// # Arguments
/// The existing live run state and configuration.
/// # Returns
/// Cloned story groups in dependency order.
/// # Examples
/// ```rust,no_run
/// # fn example(host: &impl codetether_ralph_stage::Host) {
/// let _ = codetether_ralph_stage::parallel_stages(host);
/// # }
/// ```
pub fn parallel_stages<H: Host>(host: &H) -> Vec<Vec<UserStory>> {
    // Clone stages upfront to avoid borrow issues
    let stages = codetether_ralph_core::scheduling::owned_stages(&host.inputs().state.prd);
    let total_stages = stages.len();

    info!(target: "codetether_agent::ralph::ralph_loop::parallel_plan",
        "Parallel execution: {} stages, {} max concurrent stories",
        total_stages, host.inputs().config.max_concurrent_stories
    );

    stages
}
/// Create an optional worktree manager with automatic editor opening disabled.
/// # Arguments
/// The existing workspace root and worktree-enabled setting.
/// # Returns
/// The shared manager when isolation is enabled, otherwise None.
/// # Examples
/// ```rust,no_run
/// # fn example(host: &impl codetether_ralph_stage::Host) {
/// let _ = codetether_ralph_stage::parallel_worktree_manager(host);
/// # }
/// ```
pub fn parallel_worktree_manager<H: Host>(host: &H) -> Option<Arc<WorktreeManager>> {
    // Create worktree manager if enabled
    let worktree_mgr = if host.inputs().config.worktree_enabled {
        let mgr =
            WorktreeManager::for_repo(&host.inputs().state.working_dir).without_vscode_auto_open();
        info!(target: "codetether_agent::ralph::ralph_loop::parallel_plan", "Worktree isolation enabled for parallel stories");
        Some(Arc::new(mgr))
    } else {
        None
    };

    worktree_mgr
}
