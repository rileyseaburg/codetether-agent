//! Load bounded summary nodes relative to a requested message window.
use anyhow::Result;
use codetether_context_policy::summary_index::{SummaryIndex, SummaryRange};
use rusqlite::{Connection, params};

/// Load complete summary ranges contained within a source window.
/// # Arguments
/// `db` and `id` select storage; `start` and `end` bound the message window.
/// # Returns
/// Up to 128 nodes, rebased to the supplied window's start.
/// # Errors
/// SQLite queries or summary JSON decoding fail.
/// # Examples
/// ```rust,no_run
/// # fn example(db: &rusqlite::Connection) -> anyhow::Result<()> {
/// let index = codetether_session_store::summaries::read(db, "session-123", 0, 100)?;
/// # Ok(()) }
/// ```
pub fn read(db: &Connection, id: &str, start: usize, end: usize) -> Result<SummaryIndex> {
    let mut query = db.prepare(
        "SELECT seq,end_seq,body FROM projections
        WHERE session_id=?1 AND name='manual_summary' AND seq>=?2 AND seq<?3
        AND end_seq<=?3 AND end_seq>seq ORDER BY seq DESC LIMIT 128",
    )?;
    let rows = query.query_map(params![id, start as i64, end as i64], |r| {
        Ok((
            r.get::<_, usize>(0)?,
            r.get::<_, usize>(1)?,
            r.get::<_, String>(2)?,
        ))
    })?;
    let mut index = SummaryIndex::new();
    for row in rows {
        let (from, to, body) = row?;
        if let Some(range) = SummaryRange::new(from - start, to - start) {
            index.insert(range, serde_json::from_str(&body)?);
        }
    }
    Ok(index)
}
