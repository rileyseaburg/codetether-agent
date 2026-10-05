//! Preserve the existing eight-excerpt hard-constraint header across window eviction.
mod capture;
mod text;
use crate::provider::Message;
use crate::session::{Session, pages::PageKind};
use anyhow::Result;
pub(super) use capture::evicted;
use rusqlite::{Connection, Transaction, params};
pub(super) use text::excerpt;
pub(super) fn record(
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
pub(super) fn read(db: &Connection, id: &str, before: usize) -> Result<Vec<(usize, String)>> {
    let mut query = db.prepare(
        "SELECT seq,excerpt FROM constraints WHERE session_id=?1 AND seq<?2 ORDER BY seq LIMIT 8",
    )?;
    Ok(query
        .query_map(params![id, before as i64], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<_, _>>()?)
}
pub(crate) fn prefix(session: &Session) -> Vec<String> {
    session
        .storage
        .0
        .lock()
        .unwrap()
        .constraints
        .iter()
        .map(|(seq, text)| format!("- turn {seq}: {text}"))
        .collect()
}
