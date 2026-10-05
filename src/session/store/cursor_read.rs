//! Read a bounded delta and its invalidation generation in one SQLite snapshot.
use super::cursor::{Delivery, Ticket};
use anyhow::Result;
use rusqlite::params;
use std::path::Path;
mod hydrate;
pub(super) fn next(path: &Path, id: &str, name: &str, limit: usize) -> Result<Option<Delivery>> {
    anyhow::ensure!(limit > 0, "consumer batch must be nonzero");
    super::migration::ensure(path)?;
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
    let session = hydrate::session(&tx, &ticket, header, count, revision)?;
    Ok(Some(Delivery { session, ticket }))
}
