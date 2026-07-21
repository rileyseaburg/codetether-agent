//! Managed Git worktree allocation for mux agent windows.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub(in crate::mux) async fn workspace(
    session: &str,
    slot: u64,
    requested: &Path,
) -> Result<PathBuf> {
    let requested = tokio::fs::canonicalize(requested)
        .await
        .context("resolve mux workspace")?;
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

fn primary_checkout(repo: &Path) -> PathBuf {
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

#[cfg(test)]
#[path = "isolation_tests.rs"]
mod tests;
