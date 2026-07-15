//! Workspace clone lock helpers.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};

use super::{env_u64, git_clone_base_dir};

pub(super) fn workspace_clone_lock_path(workspace_id: &str) -> PathBuf {
    let safe_id: String = workspace_id
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect();
    git_clone_base_dir().join(format!(".{safe_id}.clone.lock"))
}

pub(super) async fn acquire_workspace_clone_lock(lock_path: &Path) -> Result<tokio::fs::File> {
    if let Some(parent) = lock_path.parent() {
        tokio::fs::create_dir_all(parent).await.with_context(|| {
            format!("Failed to create clone lock directory {}", parent.display())
        })?;
    }
    let stale_after = Duration::from_secs(env_u64("CODETETHER_WORKER_CLONE_LOCK_STALE_SECS", 1800));
    loop {
        match tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(lock_path)
            .await
        {
            Ok(file) => return Ok(file),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                if super::clone_lock_stale::workspace_clone_lock_is_stale(lock_path, stale_after)
                    .await
                {
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
