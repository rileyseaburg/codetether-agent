//! Persist a sidecar before saving its session metadata.
use super::Storage;
use crate::RunCheckpoint;
use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

/// Persist a checkpoint sidecar and then save its session metadata.
/// # Arguments
/// `session` supplies native storage; `checkpoint` is the value to persist.
/// # Returns
/// The sidecar path after both writes succeed.
/// # Errors
/// Path validation, directory creation, serialization, sidecar writing or native saving fails.
/// # Examples
/// ```rust,no_run
/// use codetether_run_checkpoint::storage::{Storage, save};
/// # async fn example<S: Storage>(session: &mut S, checkpoint: codetether_run_checkpoint::RunCheckpoint) -> anyhow::Result<()> {
/// let result = save(session, checkpoint).await?;
/// # Ok(()) }
/// ```
pub async fn save<S: Storage>(session: &mut S, checkpoint: RunCheckpoint) -> Result<PathBuf> {
    let path = super::path::checkpoint_path::<S>(session.id())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    session.set_checkpoint(Some(checkpoint.clone()));
    fs::write(&path, serde_json::to_vec_pretty(&checkpoint)?).await?;
    session.save_session().await?;
    Ok(path)
}
