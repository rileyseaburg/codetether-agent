//! Create and isolate one story worktree, retaining fallback errors.
use super::*;
pub(crate) async fn create_story_worktree(
    mgr: Arc<WorktreeManager>,
    story_id: String,
) -> (String, Option<WorktreeInfo>) {
    let slug = story_id.to_lowercase().replace("-", "_");
    match mgr.create(&slug).await {
        Ok(wt) => {
            if let Err(e) = mgr.inject_workspace_stub(&wt.path) {
                warn!(target: "codetether_agent::ralph::ralph_loop::parallel_create_worktree",
                    story_id = %story_id,
                    error = %e,
                    "Failed to inject workspace stub"
                );
            }
            info!(target: "codetether_agent::ralph::ralph_loop::parallel_create_worktree",
                story_id = %story_id,
                worktree_path = %wt.path.display(),
                "Pre-created worktree for story"
            );
            (story_id, Some(wt))
        }
        Err(e) => {
            warn!(target: "codetether_agent::ralph::ralph_loop::parallel_create_worktree",
                story_id = %story_id,
                error = %e,
                "Failed to pre-create worktree, will use main directory"
            );
            (story_id, None)
        }
    }
}
