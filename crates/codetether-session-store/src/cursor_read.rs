//! Read a bounded delta and its invalidation generation in one SQLite snapshot.
use crate::{Delivery, Ticket};
use anyhow::Result;
use rusqlite::params;
use std::path::Path;
/// Read and decode a bounded consumer delta within one database snapshot.
/// # Arguments
/// `path`, `id`, and `name` identify storage and consumer; `limit` is nonzero.
/// `prepare` performs native import before opening the database; `hydrate`
/// decodes the snapshot using the supplied transaction and range metadata.
/// # Returns
/// The decoded payload and acknowledgement token, or `None` at the stream end.
/// # Errors
/// Rejects a zero limit and propagates import, SQLite and decoding failures.
/// # Examples
/// ```rust,no_run
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let delta = codetether_session_store::cursor_read::next(
///     path, "session-123", "recall", 4,
///     |_| Ok(()), |_tx, _ticket, header, _count, _revision| Ok(header),
/// )?;
/// # Ok(()) }
/// ```
pub fn next<S>(
    path: &Path,
    id: &str,
    name: &str,
    limit: usize,
    prepare: impl FnOnce(&Path) -> Result<()>,
    hydrate: impl FnOnce(&rusqlite::Transaction<'_>, &Ticket, String, usize, i64) -> Result<S>,
) -> Result<Option<Delivery<S>>> {
    anyhow::ensure!(limit > 0, "consumer batch must be nonzero");
    prepare(path)?;
    let db = super::connection::open(path)?;
    db.execute(
        "INSERT OR IGNORE INTO consumers VALUES (?1,?2,0,0)",
        params![id, name],
    )?;
    let tx = db.unchecked_transaction()?;
    let (from, generation): (usize, i64) = tx.query_row(
        "SELECT seq,revision FROM consumers WHERE session_id=?1 AND name=?2",
        params![id, name],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    let (revision, header, count, _) = super::read::header(&tx, id)?;
    if from >= count {
        return Ok(None);
    }
    let to = from.saturating_add(limit).min(count);
    let from = if name == "recall" { from / 4 * 4 } else { from };
    let ticket = Ticket {
        path: path.into(),
        id: id.into(),
        name: name.into(),
        from,
        to,
        generation,
    };
    let session = hydrate(&tx, &ticket, header, count, revision)?;
    Ok(Some(Delivery { session, ticket }))
}
