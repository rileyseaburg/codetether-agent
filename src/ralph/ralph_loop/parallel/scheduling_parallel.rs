//! Application stage execution retains task handles through finalization.
use super::*;
#[async_trait::async_trait]
impl codetether_ralph_core::scheduling::ParallelHost for RalphLoop {
    type Manager = Arc<WorktreeManager>;
    fn stages(&self) -> Vec<Vec<UserStory>> {
        self.parallel_stages()
    }
    fn manager(&self) -> Option<Self::Manager> {
        self.parallel_worktree_manager()
    }
    fn begin(&mut self, index: usize, total: usize, stories: &[UserStory]) -> bool {
        self.begin_parallel_stage(index, total, stories)
    }
    async fn execute_stage(
        &mut self,
        stories: Vec<UserStory>,
        worktree_mgr: &Option<Self::Manager>,
    ) -> anyhow::Result<()> {
        let worktrees_by_story =
            Self::precreate_stage_worktrees(worktree_mgr.as_ref(), &stories).await;
        let stage = self.spawn_parallel_stage(stories, worktree_mgr, worktrees_by_story);
        self.collect_parallel_results(
            stage.handles,
            &stage.provider,
            &stage.model,
            &stage.working_dir,
        )
        .await;
        self.finish_parallel_stage().await?;
        Ok(())
    }
}
