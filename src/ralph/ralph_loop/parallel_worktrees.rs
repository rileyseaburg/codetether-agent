//! Pre-create stage worktrees with the original bounded concurrency.
use super::*;
impl RalphLoop {
    pub(super) async fn precreate_stage_worktrees(
        worktree_mgr: Option<&Arc<WorktreeManager>>,
        stories: &[UserStory],
    ) -> HashMap<String, crate::worktree::WorktreeInfo> {
        // Pre-create all worktrees for this stage in parallel *before*
        // spawning the story tasks. `git worktree add` serializes on the
        // `.git/worktrees` lock, so doing this up front (bounded to 8)
        // drains the lock contention in one burst rather than racing it
        // against every story's first tool call.
        let mut worktrees_by_story: HashMap<String, crate::worktree::WorktreeInfo> = HashMap::new();
        if let Some(mgr) = worktree_mgr {
            let ids: Vec<String> = stories.iter().map(|s| s.id.clone()).collect();
            let created: Vec<(String, Option<crate::worktree::WorktreeInfo>)> =
                stream::iter(ids.into_iter())
                    .map(|story_id| Self::create_story_worktree(Arc::clone(mgr), story_id))
                    .buffer_unordered(8)
                    .collect()
                    .await;
            for (sid, wt) in created {
                if let Some(wt) = wt {
                    worktrees_by_story.insert(sid, wt);
                }
            }
        }

        worktrees_by_story
    }
}
