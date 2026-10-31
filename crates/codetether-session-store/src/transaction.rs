//! Revision checks and records commit atomically under SQLite's writer lock.
use super::batch::Batch;
use anyhow::Result;
mod apply;
pub(crate) use apply::run as commit_on;
mod guard;
mod header;
mod pins;
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
    commit_with_pins(path, batch, &[])
}

pub(crate) fn commit_with_pins(
    path: &Path,
    batch: &Batch,
    edits: &[crate::pin_edits::PinSource],
) -> Result<i64> {
    let batch = batch.clone();
    let edits = edits.to_vec();
    crate::coordinator::write(path, move |db| commit_on(db, &batch, &edits))
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
