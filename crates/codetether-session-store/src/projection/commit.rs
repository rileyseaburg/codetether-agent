//! Commit materialized documents and their consumer acknowledgement.
use crate::Ticket;
use anyhow::Result;
use rusqlite::{TransactionBehavior, params};
use serde_json::Value;

/// Commit derived documents and their consumer acknowledgement atomically.
/// # Arguments
/// `ticket` selects the snapshot; `view` includes a documents array.
/// # Returns
/// Whether the range was accepted rather than invalidated or superseded.
/// # Errors
/// SQLite operations, invalid documents, serialization or the worker fail.
/// # Panics
/// Requires a Tokio runtime for the blocking worker.
/// # Examples
/// ```rust,no_run
/// # async fn example(ticket: codetether_session_store::Ticket, view: serde_json::Value) -> anyhow::Result<()> {
/// let result = codetether_session_store::projection::commit(ticket, view).await?;
/// # Ok(()) }
/// ```
pub async fn commit(ticket: Ticket, mut view: Value) -> Result<bool> {
    tokio::task::spawn_blocking(move || {
        let mut db = crate::connection::open(&ticket.path)?;
        // Reserve the writer before reading; deferred upgrades can bypass the busy timeout.
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (generation, position): (i64, usize) = tx.query_row("SELECT revision,seq FROM consumers WHERE session_id=?1 AND name=?2",
            params![ticket.id,ticket.name], |r| Ok((r.get(0)?, r.get(1)?)))?;
        if generation != ticket.generation || position > ticket.to { return Ok(false); }
        let docs = view["documents"].take();
        for doc in docs.as_array().ok_or_else(|| anyhow::anyhow!("view needs documents"))? {
            tx.execute("INSERT INTO projections VALUES (?1,?2,?3,?4,?5)
                ON CONFLICT(session_id,name,seq) DO UPDATE SET end_seq=excluded.end_seq,body=excluded.body",
                params![ticket.id,ticket.name,doc["start"].as_u64(),doc["end"].as_u64(),serde_json::to_string(doc)?])?;
        }
        tx.execute("INSERT INTO views VALUES (?1,?2,?3) ON CONFLICT(session_id,name) DO UPDATE SET header=excluded.header",
            params![ticket.id,ticket.name,serde_json::to_string(&view)?])?;
        tx.execute("UPDATE consumers SET seq=?3 WHERE session_id=?1 AND name=?2 AND seq<=?3",
            params![ticket.id,ticket.name,ticket.to as i64])?;
        tx.commit()?;
        Ok(true)
    }).await?
}
