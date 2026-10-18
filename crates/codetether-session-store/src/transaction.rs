//! Revision checks and records commit atomically under SQLite's writer lock.
use super::batch::Batch;
use anyhow::Result;
use rusqlite::{TransactionBehavior, params};
mod guard;
mod header;
use std::path::Path;
/// Commit an optimistic delta atomically, recognizing acknowledged retries.
/// # Arguments
/// `path` locates the database; `batch` carries the expected revision and delta.
/// # Returns
/// The committed revision.
/// # Errors
/// Opening, revision checks, record/index writes or transaction commit fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::transaction::commit;
/// # fn example(path: &std::path::Path, batch: &codetether_session_store::Batch) -> anyhow::Result<()> {
/// let result = commit(path, batch)?;
/// # Ok(()) }
/// ```
pub fn commit(path: &Path, batch: &Batch) -> Result<i64> {
    let mut db = super::connection::open(path)?;
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let nonce = super::identity::nonce(batch);
    if let Some(revision) = guard::retry(&tx, &batch.id, &nonce)? {
        return Ok(revision);
    }
    let old_count = guard::check(&tx, batch)?;
    let revision = batch.revision + 1;
    header::write(&tx, batch, revision, old_count)?;
    super::records::write(&tx, &batch.id, 0, &batch.messages)?;
    super::records::write(&tx, &batch.id, 1, &batch.tools)?;
    super::tool_state::apply(&tx, batch)?;
    super::events::append(&tx, batch, revision)?;
    tx.execute(
        "INSERT INTO commits VALUES (?1,?2,?3)",
        params![batch.id, revision, nonce],
    )?;
    tx.commit()?;
    Ok(revision)
}

/// Read the current persisted revision of a session.
/// # Arguments
/// `path` locates the database; `id` selects the session.
/// # Returns
/// The persisted revision.
/// # Errors
/// Opening the database or looking up the session fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::transaction::revision;
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let result = revision(path, "session-123")?;
/// # Ok(()) }
/// ```
pub fn revision(path: &Path, id: &str) -> Result<i64> {
    let db = super::connection::open(path)?;
    Ok(
        db.query_row("SELECT revision FROM sessions WHERE id=?1", [id], |row| {
            row.get(0)
        })?,
    )
}
