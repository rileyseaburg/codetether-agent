//! Commit canonical metadata before atomically publishing its fallback sidecar.
use super::Storage;
use crate::RunCheckpoint;
use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::fs;

/// Save canonical checkpoint metadata, then atomically replace the fallback sidecar.
/// # Arguments
/// `session` supplies native storage; `checkpoint` is the value to persist.
/// # Returns
/// The sidecar path after both writes succeed.
/// # Errors
/// Path validation, directory creation, serialization, sidecar writing or native saving fails.
/// Metadata-save failures restore the previous in-memory checkpoint and leave
/// the sidecar unchanged. Sidecar failures retain the committed canonical checkpoint.
/// # Examples
/// ```rust,no_run
/// use codetether_run_checkpoint::storage::{Storage, save};
/// # async fn example<S: Storage>(session: &mut S, checkpoint: codetether_run_checkpoint::RunCheckpoint) -> anyhow::Result<()> {
/// let result = save(session, checkpoint).await?;
/// # Ok(()) }
/// ```
pub async fn save<S: Storage>(session: &mut S, checkpoint: RunCheckpoint) -> Result<PathBuf> {
    anyhow::ensure!(
        checkpoint.session_id == session.id(),
        "checkpoint belongs to another session"
    );
    let path = super::path::checkpoint_path::<S>(session.id())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    let previous = session.checkpoint().cloned();
    let previously_cleared = session.checkpoint_cleared();
    session.set_checkpoint(Some(checkpoint.clone()));
    session.set_checkpoint_cleared(false);
    if let Err(error) = session.save_session().await {
        session.set_checkpoint(previous);
        session.set_checkpoint_cleared(previously_cleared);
        return Err(error.context("canonical checkpoint save failed; sidecar unchanged"));
    }
    super::replace::replace(&path, &checkpoint)
        .await
        .context("canonical checkpoint committed but sidecar publication failed")?;
    Ok(path)
}
