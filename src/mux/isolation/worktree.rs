//! Managed Git worktree allocation for isolated mux windows.

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Create a managed worktree for `session`/`slot` mirroring `requested`.
///
/// Non-repository directories are returned unchanged because there is no
/// repository to branch from.
pub(super) async fn allocate(session: &str, slot: u64, requested: PathBuf) -> Result<PathBuf> {
    let Some(repo) = crate::provenance::repo_root(&requested)? else {
        return Ok(requested);
    };
    let relative = requested.strip_prefix(&repo).unwrap_or(Path::new(""));
    let repo = primary_checkout(&repo);
    let suffix = &uuid::Uuid::new_v4().simple().to_string()[..8];
    let name = format!("mux-{session}-{slot}-{suffix}");
    let manager = crate::worktree::WorktreeManager::for_repo(repo).without_vscode_auto_open();
    let worktree = manager.create(&name).await?;
    Ok(worktree.path.join(relative))
}

/// Resolve a managed worktree path back to the primary checkout.
pub(super) fn primary_checkout(repo: &Path) -> PathBuf {
    for ancestor in repo.ancestors() {
        if ancestor
            .file_name()
            .is_some_and(|name| name == ".codetether-worktrees")
        {
            return ancestor.parent().unwrap_or(repo).to_path_buf();
        }
    }
    repo.to_path_buf()
}
