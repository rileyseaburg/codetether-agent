//! Delete local (and optionally remote) git branches.

use anyhow::Result;
use std::path::Path;
use tokio::process::Command;

/// Delete a local branch. Ignores errors (branch may already be gone).
pub async fn delete_local_branch(repo_path: &Path, branch: &str) {
    let out = Command::new("git")
        .args(["branch", "-D", branch])
        .current_dir(repo_path)
        .output()
        .await;
    match out {
        Ok(o) if o.status.success() => {
            tracing::info!(branch, "Deleted local branch");
        }
        Ok(o) => {
            let msg = String::from_utf8_lossy(&o.stderr);
            tracing::debug!(branch, error = %msg, "Local branch delete skipped");
        }
        Err(e) => {
            tracing::debug!(branch, error = %e, "Local branch delete failed");
        }
    }
}

/// Delete a remote tracking branch (best-effort, no error on failure).
pub async fn delete_remote_branch(repo_path: &Path, branch: &str) {
    let out = Command::new("git")
        .args(["push", "origin", "--delete", branch])
        .current_dir(repo_path)
        .output()
        .await;
    match out {
        Ok(o) if o.status.success() => {
            tracing::info!(branch, "Deleted remote branch");
        }
        _ => {
            tracing::debug!(branch, "Remote branch delete skipped");
        }
    }
}

/// Delete both local and remote copies of a branch.
pub async fn delete_branch(repo_path: &Path, branch: &str) -> Result<()> {
    delete_local_branch(repo_path, branch).await;
    delete_remote_branch(repo_path, branch).await;
    Ok(())
}
