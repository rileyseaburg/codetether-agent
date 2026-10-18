//! Native branch lookup and session-directory binding for prompt worktrees.
use super::base_branch::current_branch;
use crate::session::Session;
pub(crate) use codetether_worktree::prompt::WorktreeState;
use std::path::Path;
pub(crate) async fn create_worktree(cwd: &Path, session: &mut Session) -> Option<WorktreeState> {
    codetether_worktree::prompt::create_worktree(cwd, current_branch, |path| {
        session.metadata.directory = Some(path);
    })
    .await
}
