//! Preserve the existing eight-excerpt hard-constraint header across window eviction.
mod capture;
mod prefix;
mod text;
use anyhow::Result;
pub use capture::evicted;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;
pub use prefix::prefix;
use rusqlite::{Connection, Transaction, params};
pub use text::excerpt;
/// Index a protected page excerpt inside the active transaction.
/// # Arguments
/// `tx`, `id`, `seq`, `message` and `page` identify a classified message.
/// # Returns
/// Unit; only constraint pages with nonempty text create an index row.
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
    if page == PageKind::Constraint
        && let Some(text) = excerpt(message)
    {
        tx.execute(
            "INSERT INTO constraints VALUES (?1,?2,?3)",
            params![id, seq, text],
        )?;
    }
    Ok(())
}
/// Read up to eight protected excerpts preceding a sequence boundary.
/// # Arguments
/// `db` and `id` select the session; `before` is the exclusive sequence limit.
/// # Returns
/// Ordered sequence numbers and excerpt text.
/// # Errors
/// SQLite reading fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::constraints::read;
/// # fn example(db: &rusqlite::Connection) -> anyhow::Result<()> {
/// let result = read(db, "session-123", 100)?;
/// # Ok(()) }
/// ```
pub fn read(db: &Connection, id: &str, before: usize) -> Result<Vec<(usize, String)>> {
    let mut query = db.prepare(
        "SELECT seq,excerpt FROM constraints WHERE session_id=?1 AND seq<?2 ORDER BY seq LIMIT 8",
    )?;
    Ok(query
        .query_map(params![id, before as i64], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<_, _>>()?)
}
