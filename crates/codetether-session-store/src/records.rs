//! Transaction-local suffix replacement: deletion is explicit, append is cheap.
use super::batch::Records;
use anyhow::Result;
use rusqlite::{Transaction, params};
/// Replace a dirty record suffix inside the current transaction.
/// # Arguments
/// `tx`, `id` and `kind` select the record stream; `records` provides the suffix.
/// # Returns
/// Unit after deletion and ordered insertion of the supplied suffix.
/// # Errors
/// SQLite writing or blob storage fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::records::write;
/// # fn example(tx: &rusqlite::Transaction<'_>, records: &codetether_session_store::Records) -> anyhow::Result<()> {
/// let result = write(tx, "session-123", 0, records)?;
/// # Ok(()) }
/// ```
pub fn write(tx: &Transaction<'_>, id: &str, kind: i64, records: &Records) -> Result<()> {
    tx.execute(
        "DELETE FROM records WHERE session_id=?1 AND kind=?2 AND seq>=?3",
        params![id, kind, records.start as i64],
    )?;
    let mut insert = tx.prepare_cached("INSERT INTO records VALUES (?1,?2,?3,?4)")?;
    for (index, body) in records.rows.iter().enumerate() {
        let body = super::blobs::put(tx, body)?;
        insert.execute(params![id, kind, (records.start + index) as i64, body])?;
    }
    Ok(())
}
