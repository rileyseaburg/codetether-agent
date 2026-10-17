//! Resolve Git hook paths and apply shared repository hook policy.
use super::git_commit_push_ops as ops;
use anyhow::Result;
use std::path::{Path, PathBuf};

async fn hooks_dir(repo_path: &Path) -> Result<PathBuf> {
    let path = ops::git(
        repo_path,
        &["rev-parse", "--path-format=absolute", "--git-path", "hooks"],
    )
    .await?;
    Ok(PathBuf::from(path.trim()))
}
/// Require all declared hooks before worker commits or pushes.
pub(super) async fn require_installed(repo_path: &Path) -> Result<()> {
    codetether_worktree::repository::require_installed(repo_path, || hooks_dir(repo_path)).await
}
#[cfg(test)]
#[path = "git_hook_gate_tests.rs"]
mod tests;
