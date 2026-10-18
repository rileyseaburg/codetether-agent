//! Invoke the existing policy-aware story merge implementation.
use super::*;
pub(super) async fn run(
    host: &mut RalphLoop,
    provider: &Arc<dyn Provider>,
    model: &str,
    working_dir: &Path,
    story: &UserStory,
    worktree: Option<&crate::worktree::WorktreeInfo>,
    manager: Option<&Arc<WorktreeManager>>,
) {
    host.merge_parallel_story(provider, model, working_dir, story, worktree, manager)
        .await;
}
