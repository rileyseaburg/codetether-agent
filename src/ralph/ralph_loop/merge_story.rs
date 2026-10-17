//! Commit a successful task and merge its isolated workspace if present.
use super::parallel::parallel_result_types::MergeContext;
use super::*;
impl RalphLoop {
    pub(super) async fn merge_parallel_story(
        &mut self,
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
        story: &UserStory,
        worktree_info: Option<&crate::worktree::WorktreeInfo>,
        worktree_mgr: Option<&Arc<WorktreeManager>>,
    ) {
        info!("Story {} passed quality checks!", story.id);

        // Commit in worktree first
        if let Some(wt) = worktree_info {
            let _ = self.commit_in_dir(&wt.path, story);
        }

        // Merge worktree back to main
        if let (Some(wt), Some(mgr)) = (worktree_info, worktree_mgr) {
            let ctx = MergeContext {
                provider,
                model,
                working_dir,
                story,
                wt,
                mgr,
            };
            let result = mgr.merge(&wt.name).await;
            self.apply_worktree_merge(&ctx, result).await;
        } else {
            if !self
                .complete_story_without_worktree(story, working_dir)
                .await
            {
                return;
            }
        }
    }
}
