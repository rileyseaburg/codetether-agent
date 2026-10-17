//! Locate and remove a held clone lock by its file identity.
use std::path::PathBuf;
/// Release a held clone lock with best-effort inode-based cleanup on Unix.
///
/// # Arguments
/// * `lock` - Open lock file retained by the clone operation.
/// * `_resolve_base` - Resolve the search directory when metadata can be read on Unix.
/// # Returns
/// Unit after dropping the file; cleanup failures leave stale recovery to acquisition.
/// # Examples
/// ```rust,no_run
/// # async fn example(lock: tokio::fs::File) {
/// use codetether_worktree::repository::clone_lock::release_workspace_clone_lock;
/// release_workspace_clone_lock(lock, || "clones".into()).await;
/// # }
/// ```

pub async fn release_workspace_clone_lock(
    lock: tokio::fs::File,
    _resolve_base: impl FnOnce() -> PathBuf,
) {
    if let Ok(metadata) = lock.metadata().await {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Ok(mut entries) = tokio::fs::read_dir(_resolve_base()).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(entry_metadata) = entry.metadata().await
                        && entry_metadata.ino() == metadata.ino()
                        && entry_metadata.dev() == metadata.dev()
                    {
                        let _ = tokio::fs::remove_file(entry.path()).await;
                        drop(lock);
                        return;
                    }
                }
            }
        }
    }
    #[cfg(not(unix))]
    {
        tracing::warn!(target: "codetether_agent::a2a::worker::clone_lock_release", "Clone lock cleanup skipped on non-Unix; lock file may linger");
    }
    drop(lock);
}
