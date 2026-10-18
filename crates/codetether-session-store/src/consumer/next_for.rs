//! Resolve a session and read its next consumer batch on the blocking pool.
use crate::{Delivery, legacy::Legacy};
/// Read the next consumer range for a durable session identity.
/// # Arguments
/// `id` selects the session, `name` selects its cursor, and `limit` bounds the batch.
/// # Returns
/// A snapshot and acknowledgement ticket, or `None` at the end of the stream.
/// # Errors
/// Propagates location, migration, database, decoding and blocking-task errors.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_store::legacy::Legacy + Send + 'static>() -> anyhow::Result<()> {
/// let _delivery = codetether_session_store::consumer::next_for::<S>("session", "recall", 8).await?;
/// # Ok(()) }
/// ```
pub async fn next_for<S: Legacy + Send + 'static>(
    id: &str,
    name: &str,
    limit: usize,
) -> anyhow::Result<Option<Delivery<S>>> {
    let path = crate::lifecycle::session_path(id)?;
    let id = id.to_owned();
    let name = name.to_owned();
    tokio::task::spawn_blocking(move || super::next::<S>(&path, &id, &name, limit)).await?
}
