//! Exclusive clone lock acquisition with stale-file recovery.
use anyhow::{Context, Result};
use std::{path::Path, time::Duration};
/// Wait for an exclusive clone lock, removing stale files when necessary.
///
/// # Arguments
/// * `lock_path` - Lock file to create exclusively.
/// * `stale_seconds` - Resolve the stale threshold after creating the parent directory.
/// # Returns
/// The opened lock file, which the caller retains while cloning.
/// # Errors
/// Returns directory creation and lock-open errors other than existing locks.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_worktree::repository::clone_lock::acquire_workspace_clone_lock;
/// let lock = acquire_workspace_clone_lock(std::path::Path::new("clones/.one.clone.lock"), || 1800).await?;
/// # Ok(()) }
/// ```

pub async fn acquire_workspace_clone_lock(
    lock_path: &Path,
    stale_seconds: impl FnOnce() -> u64,
) -> Result<tokio::fs::File> {
    if let Some(parent) = lock_path.parent() {
        tokio::fs::create_dir_all(parent).await.with_context(|| {
            format!("Failed to create clone lock directory {}", parent.display())
        })?;
    }
    let stale_after = Duration::from_secs(stale_seconds());
    loop {
        match tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(lock_path)
            .await
        {
            Ok(file) => return Ok(file),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                if super::stale::workspace_clone_lock_is_stale(lock_path, stale_after).await {
                    let _ = tokio::fs::remove_file(lock_path).await;
                } else {
                    tokio::time::sleep(Duration::from_millis(250)).await;
                }
            }
            Err(error) => {
                return Err(error).with_context(|| {
                    format!(
                        "Failed to acquire workspace clone lock {}",
                        lock_path.display()
                    )
                });
            }
        }
    }
}
