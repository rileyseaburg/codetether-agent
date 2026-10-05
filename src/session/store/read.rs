//! Indexed range reads visit only requested records, in sequence order.
use anyhow::Result;
use rusqlite::{Connection, params};
pub(super) fn read_window(
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
pub(super) fn header(db: &Connection, id: &str) -> Result<(i64, String, usize, usize)> {
    Ok(db.query_row(
        "SELECT revision,header,message_count,tool_count FROM sessions WHERE id=?1",
        [id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )?)
}
pub(super) fn contains(db: &Connection, id: &str) -> Result<bool> {
    Ok(db.query_row(
        "SELECT EXISTS(SELECT 1 FROM sessions WHERE id=?1)",
        [id],
        |r| r.get(0),
    )?)
}
