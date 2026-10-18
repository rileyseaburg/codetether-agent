//! Prefer embedded checkpoint metadata before reading a sidecar.
use super::Storage;
use crate::RunCheckpoint;
use anyhow::Result;
use tokio::fs;

/// Read embedded checkpoint metadata before falling back to its sidecar.
/// # Arguments
/// `session` supplies identity and any embedded checkpoint.
/// # Returns
/// The checkpoint, or `None` when no sidecar or embedded value exists.
/// # Errors
/// Path validation, reading or checkpoint decoding fails.
/// # Examples
/// ```rust,no_run
/// use codetether_run_checkpoint::storage::{Storage, load};
/// # async fn example<S: Storage>(session: &S) -> anyhow::Result<()> {
/// let result = load(session).await?;
/// # Ok(()) }
/// ```
pub async fn load<S: Storage>(session: &S) -> Result<Option<RunCheckpoint>> {
    if let Some(checkpoint) = session.checkpoint() {
        return Ok(Some(checkpoint.clone()));
    }
    let path = super::path::checkpoint_path::<S>(session.id())?;
    match fs::read_to_string(path).await {
        Ok(raw) => Ok(Some(serde_json::from_str(&raw)?)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.into()),
    }
}
