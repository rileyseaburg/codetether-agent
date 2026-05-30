//! Git branch helpers for worker task commits.

use anyhow::{Context, Result};
use std::path::Path;

use super::clone_git::run_git_command_at;

pub(super) async fn ensure_metadata_checked_out(
    repo_path: Option<&Path>,
    branch: Option<String>,
) -> Result<()> {
    if let (Some(repo_path), Some(branch)) = (repo_path, branch) {
        ensure_checked_out(repo_path, &branch).await?;
    }
    Ok(())
}

async fn ensure_checked_out(repo_path: &Path, branch: &str) -> Result<()> {
    if branch.trim().is_empty() || !repo_path.join(".git").exists() {
        return Ok(());
    }
    if current_branch(repo_path).await?.trim() == branch {
        return Ok(());
    }
    run_git_command_at(
        Some(repo_path),
        vec!["checkout".into(), "-B".into(), branch.to_string()],
    )
    .await
    .with_context(|| format!("Failed to checkout task branch {branch}"))?;
    Ok(())
}

async fn current_branch(repo_path: &Path) -> Result<String> {
    run_git_command_at(
        Some(repo_path),
        vec!["rev-parse".into(), "--abbrev-ref".into(), "HEAD".into()],
    )
    .await
}
