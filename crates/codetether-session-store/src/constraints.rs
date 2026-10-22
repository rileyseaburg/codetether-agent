//! Index complete source-backed constraints and their explicit supersessions.
mod capture;
mod prefix;
use anyhow::Result;
pub use capture::evicted;
use codetether_context_policy::pages::PageKind;
pub use codetether_context_policy::state_header::ConstraintRecord;
use codetether_provider_core::Message;
pub use prefix::prefix;
use rusqlite::{Connection, Transaction, params};
/// Index a complete pinned source inside the active transaction.
/// # Arguments
/// `tx`, `id`, `seq`, `message` and `page` identify a classified message.
/// # Returns
/// Unit; only active or superseded constraint pages with text create a row.
/// # Errors
/// SQLite insertion fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::constraints::record;
/// # fn example(tx: &rusqlite::Transaction<'_>, message: &codetether_provider_core::Message) -> anyhow::Result<()> {
/// let result = record(tx, "session-123", 0, message, codetether_context_policy::pages::PageKind::Constraint)?;
/// # Ok(()) }
/// ```
pub fn record(
    tx: &Transaction<'_>,
    id: &str,
    seq: i64,
    message: &Message,
    page: PageKind,
) -> Result<()> {
    if let Some(record) = ConstraintRecord::from_message(usize::try_from(seq)?, message, page) {
        tx.execute(
            "INSERT INTO constraints VALUES (?1,?2,?3)",
            params![id, seq, serde_json::to_string(&record)?],
        )?;
    }
    Ok(())
}
/// Read all complete pinned sources preceding a sequence boundary.
/// # Arguments
/// `db` and `id` select the session; `before` is the exclusive sequence limit.
/// # Returns
/// Ordered source records, including supersession metadata.
/// # Errors
/// SQLite reading fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::constraints::read;
/// # fn example(db: &rusqlite::Connection) -> anyhow::Result<()> {
/// let result = read(db, "session-123", 100)?;
/// # Ok(()) }
/// ```
pub fn read(db: &Connection, id: &str, before: usize) -> Result<Vec<ConstraintRecord>> {
    let mut query = db.prepare(
        "SELECT seq,excerpt FROM constraints WHERE session_id=?1 AND seq<?2 ORDER BY seq",
    )?;
    query
        .query_map(
            params![id, i64::try_from(before).unwrap_or(i64::MAX)],
            |row| Ok((row.get::<_, usize>(0)?, row.get::<_, String>(1)?)),
        )?
        .map(|row| {
            let (seq, text) = row?;
            let record: ConstraintRecord = serde_json::from_str(&text)?;
            anyhow::ensure!(
                record.source_turn == seq && record.has_valid_hash(),
                "constraint source integrity mismatch at turn {seq}"
            );
            anyhow::ensure!(
                record.superseded_by.is_none_or(|next| next > seq),
                "invalid constraint supersession at turn {seq}"
            );
            Ok(record)
        })
        .collect()
}
