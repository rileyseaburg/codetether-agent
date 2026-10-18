//! Ordered index, record and locator deletion.
use super::Host;
use anyhow::Result;
/// Delete the session, accepting a locator that is already absent.
///
/// # Arguments
/// * `id` — Durable session identity to remove.
/// # Returns
/// Success after indexes, records and locator cleanup.
/// # Errors
/// Identity, recall-index, storage or non-missing locator removal errors.
/// # Examples
/// ```no_run
/// # async fn run<H: codetether_session_store::lifecycle::Host>(id: &str) -> anyhow::Result<()> {
/// let _result = codetether_session_store::lifecycle::delete::<H>(id).await?;
/// # Ok(())
/// # }
/// ```
pub async fn delete<H: Host>(id: &str) -> Result<()> {
    let path = super::session_path(id)?;
    H::notify_removed(id).await;
    H::remove_recall(id).await?;
    crate::delete::run(&path, id).await?;
    match tokio::fs::remove_file(&path).await {
        Ok(()) => {
            codetether_session_location::forget(id);
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            codetether_session_location::forget(id);
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
