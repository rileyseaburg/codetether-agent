//! Explicit summary cache writes are independent of the session's append path.
use crate::session::{
    Session,
    index::{SummaryIndex, SummaryNode, SummaryRange},
};
use anyhow::Result;
use rusqlite::{Connection, params};
pub(crate) async fn put(session: &Session, range: SummaryRange, node: &SummaryNode) -> Result<()> {
    let state = session.storage.0.lock().unwrap().clone();
    let body = serde_json::to_string(node)?;
    tokio::task::spawn_blocking(move || {
        let db = super::connection::open(&state.path)?;
        let tx = db.unchecked_transaction()?;
        let revision: i64 = tx.query_row("SELECT revision FROM sessions WHERE id=?1", [&state.id], |r| r.get(0))?;
        anyhow::ensure!(revision == state.revision, "SESSION_REVISION_CONFLICT: summary source changed");
        tx.execute("INSERT INTO projections VALUES (?1,'manual_summary',?2,?3,?4)
            ON CONFLICT(session_id,name,seq) DO UPDATE SET end_seq=excluded.end_seq,body=excluded.body",
            params![state.id,(state.message_start+range.start) as i64,(state.message_start+range.end) as i64,body])?;
        tx.commit()?;
        anyhow::Ok(())
    }).await?
}
pub(super) fn read(db: &Connection, id: &str, start: usize, end: usize) -> Result<SummaryIndex> {
    let mut query = db.prepare("SELECT seq,end_seq,body FROM projections
        WHERE session_id=?1 AND name='manual_summary' AND seq>=?2 AND seq<?3 ORDER BY seq DESC LIMIT 128")?;
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
        if to > end {
            continue;
        }
        if let Some(range) = SummaryRange::new(from - start, to - start) {
            index.insert(range, serde_json::from_str(&body)?);
        }
    }
    Ok(index)
}
