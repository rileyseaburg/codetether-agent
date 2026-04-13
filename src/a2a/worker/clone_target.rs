//! Clone target preparation for worker-managed repositories.
//!
//! This module makes sure a clone destination is safe to use before the worker
//! runs `git clone`, including cleanup of stale empty directories.
//!
//! # Examples
//!
//! ```ignore
//! prepare_clone_target(std::path::Path::new("/tmp/repo")).await?;
//! ```

use anyhow::{Context, Result};
use std::path::Path;

/// Ensures the clone target is suitable for a new repository clone.
///
/// Existing git repositories are left untouched. Empty directories are
/// removed, while non-empty non-repositories are rejected.
///
/// # Examples
///
/// ```ignore
/// prepare_clone_target(std::path::Path::new("/tmp/repo")).await?;
/// ```
pub(super) async fn prepare_clone_target(repo_path: &Path) -> Result<()> {
    if !repo_path.exists() || repo_path.join(".git").exists() {
        return Ok(());
    }
    remove_empty_clone_target(repo_path).await?;
    if repo_path.exists() {
        anyhow::bail!(
            "Clone target {} already exists but is not a Git repository",
            repo_path.display()
        );
    }
    Ok(())
}

async fn remove_empty_clone_target(repo_path: &Path) -> Result<()> {
    let mut entries = tokio::fs::read_dir(repo_path)
        .await
        .with_context(|| format!("Failed to inspect clone target {}", repo_path.display()))?;
    if entries.next_entry().await?.is_none() {
        tokio::fs::remove_dir(repo_path).await.with_context(|| {
            format!(
                "Failed to remove empty clone target {}",
                repo_path.display()
            )
        })?;
    }
    Ok(())
}
