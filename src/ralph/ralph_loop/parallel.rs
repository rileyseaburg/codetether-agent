//! Coordinate dependency stages, isolated task spawning and result collection.
use super::*;
impl RalphLoop {
    /// Run stories in parallel by stage with worktree isolation
    pub(super) async fn run_parallel(&mut self) -> anyhow::Result<()> {
        let stages = self.parallel_stages();
        let total_stages = stages.len();
        let worktree_mgr = self.parallel_worktree_manager();
        for (stage_idx, stage_stories) in stages.into_iter().enumerate() {
            if !self.begin_parallel_stage(stage_idx, total_stages, &stage_stories) {
                break;
            }
            // Stories are already cloned from stages()
            let stories: Vec<UserStory> = stage_stories;

            let worktrees_by_story =
                Self::precreate_stage_worktrees(worktree_mgr.as_ref(), &stories).await;
            let stage = self.spawn_parallel_stage(stories, &worktree_mgr, worktrees_by_story);
            self.collect_parallel_results(
                stage.handles,
                &stage.provider,
                &stage.model,
                &stage.working_dir,
            )
            .await;
            self.finish_parallel_stage().await?;
        }

        Ok(())
    }
}
