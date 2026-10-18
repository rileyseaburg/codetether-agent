//! Forward successful task integration to the shared merge state machine.
use super::*;
impl<H: Host> RalphLoop<H> {
    pub(crate) async fn merge_parallel_story(
        &mut self,
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
        story: &UserStory,
        worktree_info: Option<&codetether_worktree::WorktreeInfo>,
        worktree_mgr: Option<&Arc<WorktreeManager>>,
    ) {
        codetether_ralph_merge::merge_parallel_story(
            self,
            provider,
            model,
            working_dir,
            story,
            worktree_info,
            worktree_mgr,
        )
        .await;
    }
}
