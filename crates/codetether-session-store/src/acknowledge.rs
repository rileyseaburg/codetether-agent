//! Advance a consumer only for the acknowledged invalidation generation.
use crate::Ticket;
use anyhow::Result;

/// Advance a processed consumer range if its generation remains valid.
/// # Arguments
/// `ticket` is the token returned by the consumer snapshot.
/// # Returns
/// Whether one current consumer was acknowledged.
/// # Errors
/// SQLite operations or the blocking worker fail.
/// # Panics
/// Requires a Tokio runtime for the blocking worker.
/// # Examples
/// ```rust,no_run
/// # async fn example(ticket: codetether_session_store::Ticket) -> anyhow::Result<()> {
/// let result = codetether_session_store::acknowledge(ticket).await?;
/// # Ok(()) }
/// ```
pub async fn acknowledge(ticket: Ticket) -> Result<bool> {
    tokio::task::spawn_blocking(move || {
        let db = super::connection::open(&ticket.path)?;
        let rows = db.execute("UPDATE consumers SET seq=?3 WHERE session_id=?1 AND name=?2 AND revision=?4 AND seq<=?3",
            rusqlite::params![ticket.id, ticket.name, ticket.to as i64, ticket.generation])?;
        anyhow::Ok(rows == 1)
    }).await?
}
