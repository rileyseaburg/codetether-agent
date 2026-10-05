//! Transactional upgrade from the initial development schema to payload events.
use anyhow::Result;
use rusqlite::Transaction;
mod constraints;
pub(super) fn run(tx: &Transaction<'_>, version: i64) -> Result<()> {
    if version == 1 {
        let mut query = tx.prepare("PRAGMA table_info(tool_calls)")?;
        let fields = query
            .query_map([], |r| r.get::<_, String>(1))?
            .collect::<Result<Vec<_>, _>>()?;
        if !fields.is_empty() && !fields.iter().any(|field| field == "result_seq") {
            tx.execute_batch("ALTER TABLE tool_calls ADD COLUMN result_seq INTEGER")?;
        }
    }
    tx.execute_batch(super::schema::SQL)?;
    if version == 1 {
        // Existing records form a recovery baseline at their current revision.
        // Earlier revisions were not payload-journalled; do not invent them.
        tx.execute_batch(
            "INSERT OR IGNORE INTO events
            SELECT id,revision,2,0,'header',header FROM sessions;
            INSERT OR IGNORE INTO events
            SELECT r.session_id,s.revision,r.kind,r.seq,'put',r.body
            FROM records r JOIN sessions s ON s.id=r.session_id;",
        )?;
        constraints::backfill(tx)?;
    }
    Ok(())
}
