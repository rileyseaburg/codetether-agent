//! Worktree creation and post-run merge/cleanup helpers.
//!
//! Used by the TUI session runtime to isolate prompt execution.
//!
//! # Examples
//!
//! ```ignore
//! let wt = create_worktree(cwd, &mut session).await;
//! handle_worktree_result(&result, wt).await;
//! ```

use std::path::Path;

use super::base_branch::current_branch;
use crate::session::Session;
use crate::worktree::{WorktreeInfo, WorktreeManager};

/// Worktree state carried through the spawn boundary.
pub(crate) type WorktreeState = (WorktreeManager, WorktreeInfo, Option<String>);

/// Create an isolated worktree for prompt execution.
///
/// Returns `Some((mgr, wt))` on success, or `None` if
/// worktree creation fails (in which case a warning is logged
/// and the prompt runs in the main directory).
///
/// # Examples
///
/// ```ignore
/// let state = create_worktree(cwd, &mut session).await;
/// ```
pub(crate) async fn create_worktree(cwd: &Path, session: &mut Session) -> Option<WorktreeState> {
    let repo_dir = cwd.to_path_buf();
    let name = format!("tui_{}", uuid::Uuid::new_v4().simple());
    let mgr = WorktreeManager::new(repo_dir.join(".codetether-worktrees"));
    let base_branch = current_branch(cwd);
    match mgr.create(&name).await {
        Ok(wt) => {
            let _ = mgr.inject_workspace_stub(&wt.path);
            tracing::info!(
                worktree = %name,
                path = %wt.path.display(),
                "Created TUI worktree for prompt isolation"
            );
            session.metadata.directory = Some(wt.path.clone());
            Some((mgr, wt, base_branch))
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to create worktree, running in main directory");
            None
        }
    }
}
