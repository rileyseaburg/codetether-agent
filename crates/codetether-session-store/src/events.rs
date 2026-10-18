//! Immutable delta events make the current-record tables replayable, including edits.
use super::batch::Batch;
use anyhow::Result;
use rusqlite::{Transaction, params};
/// Append immutable header and suffix events for a committed delta.
/// # Arguments
/// `tx` is the writer transaction; `batch` supplies ranges; `revision` labels events.
/// # Returns
/// Unit after the event rows have been inserted.
/// # Errors
/// SQLite event insertion fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::events::append;
/// # fn example(tx: &rusqlite::Transaction<'_>, batch: &codetether_session_store::Batch) -> anyhow::Result<()> {
/// let result = append(tx, batch, 1)?;
/// # Ok(()) }
/// ```
pub fn append(tx: &Transaction<'_>, batch: &Batch, revision: i64) -> Result<()> {
    tx.execute(
        "INSERT INTO events VALUES (?1,?2,2,0,'header',?3)",
        params![batch.id, revision, batch.header],
    )?;
    for (kind, records) in [(0i64, &batch.messages), (1, &batch.tools)] {
        tx.execute(
            "INSERT INTO events VALUES (?1,?2,?3,?4,'truncate','')",
            params![batch.id, revision, kind, records.start as i64],
        )?;
        tx.execute(
            "INSERT INTO events
            SELECT session_id,?2,kind,seq,'put',body FROM records
            WHERE session_id=?1 AND kind=?3 AND seq>=?4 ORDER BY seq",
            params![batch.id, revision, kind, records.start as i64],
        )?;
    }
    Ok(())
}
