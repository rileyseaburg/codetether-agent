//! Lower-level Git operations for worker commit pushing.

use super::{clone_git::run_git_command_at, git_refspec};
use anyhow::Result;
use std::path::Path;

pub(super) async fn status(repo_path: &Path) -> Result<String> {
    git(repo_path, &["status", "--porcelain"]).await
}

pub(super) async fn git(repo_path: &Path, args: &[&str]) -> Result<String> {
    run_git_command_at(
        Some(repo_path),
        args.iter().map(|arg| arg.to_string()).collect(),
    )
    .await
}

pub(super) async fn commit(
    repo_path: &Path,
    task_id: &str,
    provenance: Option<&crate::provenance::ExecutionProvenance>,
) -> Result<()> {
    let message = format!("CodeTether task {task_id}");
    let output =
        crate::provenance::git_commit_with_provenance(repo_path, &message, provenance).await?;
    if !output.status.success() {
        anyhow::bail!(
            "git commit failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

pub(super) async fn push(repo_path: &Path, branch: &str) -> Result<()> {
    let refspec = git_refspec::push_refspec(branch)?;
    run_git_command_at(
        Some(repo_path),
        vec!["push".into(), "origin".into(), refspec],
    )
    .await?;
    Ok(())
}
