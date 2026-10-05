//! Preserve bounded legacy summary caches during the one-time import.
use crate::session::Session;
use anyhow::Result;
use std::path::Path;
pub(super) fn run(path: &Path, session: &Session) -> Result<()> {
    let db = super::connection::open(path)?;
    let tx = db.unchecked_transaction()?;
    for (range, node) in session.summary_index.entries() {
        tx.execute(
            "INSERT OR IGNORE INTO projections VALUES (?1,'manual_summary',?2,?3,?4)",
            rusqlite::params![
                session.id,
                range.start as i64,
                range.end as i64,
                serde_json::to_string(node)?
            ],
        )?;
    }
    tx.commit()?;
    Ok(())
}
