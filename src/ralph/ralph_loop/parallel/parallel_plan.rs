//! Plan dependency stages and initialize optional worktree isolation.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn parallel_stages(&self) -> Vec<Vec<UserStory>> {
        // Clone stages upfront to avoid borrow issues
        let stages: Vec<Vec<UserStory>> = self
            .state
            .prd
            .stages()
            .into_iter()
            .map(|stage| stage.into_iter().cloned().collect())
            .collect();
        let total_stages = stages.len();

        info!(target: "codetether_agent::ralph::ralph_loop::parallel_plan",
            "Parallel execution: {} stages, {} max concurrent stories",
            total_stages, self.config.max_concurrent_stories
        );

        stages
    }
    pub(in crate::ralph::ralph_loop) fn parallel_worktree_manager(
        &self,
    ) -> Option<Arc<WorktreeManager>> {
        // Create worktree manager if enabled
        let worktree_mgr = if self.config.worktree_enabled {
            let mgr = WorktreeManager::for_repo(&self.state.working_dir).without_vscode_auto_open();
            info!(target: "codetether_agent::ralph::ralph_loop::parallel_plan", "Worktree isolation enabled for parallel stories");
            Some(Arc::new(mgr))
        } else {
            None
        };

        worktree_mgr
    }
}
