//! Indexed range reads visit only requested records, in sequence order.
use anyhow::Result;
use rusqlite::{Connection, params};
/// Read a sequence range and resolve stored blob references.
/// # Arguments
/// `db`, `id` and `kind` select records; `start` is inclusive and `end` exclusive.
/// # Returns
/// Serialized record bodies in ascending sequence order.
/// # Errors
/// SQLite reading or blob integrity verification fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::read::read_window;
/// # fn example(db: &rusqlite::Connection) -> anyhow::Result<()> {
/// let result = read_window(db, "session-123", 0, 0, 10)?;
/// # Ok(()) }
/// ```
pub fn read_window(
    db: &Connection,
    id: &str,
    kind: i64,
    start: usize,
    end: usize,
) -> Result<Vec<String>> {
    let mut query = db.prepare_cached(
        "SELECT body FROM records
        WHERE session_id=?1 AND kind=?2 AND seq>=?3 AND seq<?4 ORDER BY seq",
    )?;
    let rows = query.query_map(
        params![
            id,
            kind,
            i64::try_from(start).unwrap_or(i64::MAX),
            i64::try_from(end).unwrap_or(i64::MAX)
        ],
        |row| row.get(0),
    )?;
    rows.map(|row| super::blobs::get(db, row?)).collect()
}
/// Read a session revision, header and persisted record counts.
/// # Arguments
/// `db` is the database; `id` selects the session row.
/// # Returns
/// The revision, serialized header, message count and tool count.
/// # Errors
/// The session is missing or SQLite reading fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::read::header;
/// # fn example(db: &rusqlite::Connection) -> anyhow::Result<()> {
/// let result = header(db, "session-123")?;
/// # Ok(()) }
/// ```
pub fn header(db: &Connection, id: &str) -> Result<(i64, String, usize, usize)> {
    Ok(db.query_row(
        "SELECT revision,header,message_count,tool_count FROM sessions WHERE id=?1",
        [id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )?)
}
/// Check whether a session row exists.
/// # Arguments
/// `db` is the database; `id` selects the session.
/// # Returns
/// Whether the session exists.
/// # Errors
/// SQLite reading fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::read::contains;
/// # fn example(db: &rusqlite::Connection) -> anyhow::Result<()> {
/// let result = contains(db, "session-123")?;
/// # Ok(()) }
/// ```
pub fn contains(db: &Connection, id: &str) -> Result<bool> {
    Ok(db.query_row(
        "SELECT EXISTS(SELECT 1 FROM sessions WHERE id=?1)",
        [id],
        |r| r.get(0),
    )?)
}
