//! Incremental materialized views commit with their consumer acknowledgements.
use super::cursor::Ticket;
use anyhow::Result;
use rusqlite::{OptionalExtension, TransactionBehavior, params};
use serde_json::Value;
pub(crate) async fn commit(ticket: Ticket, mut view: Value) -> Result<bool> {
    tokio::task::spawn_blocking(move || {
        let mut db = super::connection::open(&ticket.path)?;
        // Reserve the writer before reading; deferred upgrades can bypass the busy timeout.
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (generation, position): (i64, usize) = tx.query_row("SELECT revision,seq FROM consumers WHERE session_id=?1 AND name=?2",
            params![ticket.id,ticket.name], |r| Ok((r.get(0)?, r.get(1)?)))?;
        if generation != ticket.generation || position > ticket.to { return Ok(false); }
        let docs = view["documents"].take();
        for doc in docs.as_array().ok_or_else(|| anyhow::anyhow!("view needs documents"))? {
            tx.execute("INSERT INTO projections VALUES (?1,?2,?3,?4,?5)
                ON CONFLICT(session_id,name,seq) DO UPDATE SET end_seq=excluded.end_seq,body=excluded.body",
                params![ticket.id,ticket.name,doc["start"].as_u64(),doc["end"].as_u64(),serde_json::to_string(doc)?])?;
        }
        tx.execute("INSERT INTO views VALUES (?1,?2,?3) ON CONFLICT(session_id,name) DO UPDATE SET header=excluded.header",
            params![ticket.id,ticket.name,serde_json::to_string(&view)?])?;
        tx.execute("UPDATE consumers SET seq=?3 WHERE session_id=?1 AND name=?2 AND seq<=?3",
            params![ticket.id,ticket.name,ticket.to as i64])?;
        tx.commit()?;
        Ok(true)
    }).await?
}
pub(crate) async fn read(id: &str, name: &str) -> Result<Option<Value>> {
    let path = crate::session::Session::session_path(id)?;
    let id = id.to_owned();
    let name = name.to_owned();
    tokio::task::spawn_blocking(move || {
        if !path.parent().unwrap().join("sessions.sqlite3").exists() { return Ok(None); }
        let db = super::connection::open(&path)?;
        let Some(header) = db.query_row("SELECT header FROM views WHERE session_id=?1 AND name=?2", params![id,name], |r| r.get::<_,String>(0)).optional()? else { return Ok(None); };
        let mut view: Value = serde_json::from_str(&header)?;
        let mut query = db.prepare("SELECT body FROM projections WHERE session_id=?1 AND name=?2 ORDER BY seq DESC LIMIT 128")?;
        let mut docs = query.query_map(params![id,name], |r| r.get::<_,String>(0))?
            .map(|r| Ok(serde_json::from_str::<Value>(&r?)?)).collect::<Result<Vec<_>>>()?;
        docs.reverse(); view["documents"] = Value::Array(docs);
        Ok(Some(view))
    }).await?
}