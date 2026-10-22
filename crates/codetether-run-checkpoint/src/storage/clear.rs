//! Clear embedded checkpoint metadata, then remove its sidecar.
use super::Storage;
use anyhow::Result;
use tokio::fs;

/// Commit a canonical clear marker, then remove the obsolete fallback sidecar.
/// # Arguments
/// `session` is the native session whose checkpoint is being cleared.
/// # Returns
/// Unit after canonical saving and removal; a missing sidecar is accepted.
/// # Errors
/// Path validation, file removal or native saving fails. If removal fails, the
/// committed clear marker prevents the old sidecar from being loaded on restart.
/// # Examples
/// ```rust,no_run
/// use codetether_run_checkpoint::storage::{Storage, clear};
/// # async fn example<S: Storage>(session: &mut S) -> anyhow::Result<()> {
/// let result = clear(session).await?;
/// # Ok(()) }
/// ```
pub async fn clear<S: Storage>(session: &mut S) -> Result<()> {
    let path = super::path::checkpoint_path::<S>(session.id())?;
    let previous = session.checkpoint().cloned();
    let previously_cleared = session.checkpoint_cleared();
    session.set_checkpoint(None);
    session.set_checkpoint_cleared(true);
    if let Err(error) = session.save_session().await {
        session.set_checkpoint(previous);
        session.set_checkpoint_cleared(previously_cleared);
        return Err(error.context("checkpoint clear was not committed; sidecar unchanged"));
    }
    match fs::remove_file(path).await {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => {
            return Err(anyhow::Error::new(err)
                .context("checkpoint clear committed; obsolete sidecar removal failed"));
        }
    }
    Ok(())
}
