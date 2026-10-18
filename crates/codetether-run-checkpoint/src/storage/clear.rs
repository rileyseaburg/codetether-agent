//! Clear embedded checkpoint metadata, then remove its sidecar.
use super::Storage;
use anyhow::Result;
use tokio::fs;

/// Clear checkpoint metadata, remove its sidecar, and save the session.
/// # Arguments
/// `session` is the native session whose checkpoint is being cleared.
/// # Returns
/// Unit after removal and native saving; a missing sidecar is accepted.
/// # Errors
/// Path validation, file removal or native saving fails.
/// # Examples
/// ```rust,no_run
/// use codetether_run_checkpoint::storage::{Storage, clear};
/// # async fn example<S: Storage>(session: &mut S) -> anyhow::Result<()> {
/// let result = clear(session).await?;
/// # Ok(()) }
/// ```
pub async fn clear<S: Storage>(session: &mut S) -> Result<()> {
    session.set_checkpoint(None);
    let path = super::path::checkpoint_path::<S>(session.id())?;
    match fs::remove_file(path).await {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
    }
    session.save_session().await
}
