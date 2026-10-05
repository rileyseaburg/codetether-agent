//! One-time bounded-row backfill of protected excerpts during schema upgrade.
use anyhow::Result;
use rusqlite::Transaction;
pub(super) fn backfill(tx: &Transaction<'_>) -> Result<()> {
    tx.execute("DELETE FROM constraints", [])?;
    let mut query = tx.prepare("SELECT session_id,seq,body FROM records WHERE kind=0")?;
    let rows = query.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, i64>(1)?,
            r.get::<_, String>(2)?,
        ))
    })?;
    for row in rows {
        let (id, seq, body) = row?;
        let body = super::super::blobs::get(tx, body)?;
        let (message, page) = serde_json::from_str(&body)?;
        super::super::constraints::record(tx, &id, seq, &message, page)?;
    }
    Ok(())
}
