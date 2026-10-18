//! Invoke the existing policy-aware story merge implementation.
use super::*;
pub(super) async fn run<H: Host>(
    host: &mut RalphLoop<H>,
    provider: &Arc<dyn Provider>,
    model: &str,
    working_dir: &Path,
    story: &UserStory,
    worktree: Option<&codetether_worktree::WorktreeInfo>,
    manager: Option<&Arc<WorktreeManager>>,
) {
    host.merge_parallel_story(provider, model, working_dir, story, worktree, manager)
        .await;
}
