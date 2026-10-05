//! Revision checks and records commit atomically under SQLite's writer lock.
use super::batch::Batch;
use anyhow::Result;
use rusqlite::{TransactionBehavior, params};
mod guard;
mod header;
use std::path::Path;
pub(super) fn commit(path: &Path, batch: &Batch) -> Result<i64> {
    let mut db = super::connection::open(path)?;
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let nonce = super::identity::nonce(batch);
    if let Some(revision) = guard::retry(&tx, &batch.id, &nonce)? {
        return Ok(revision);
    }
    let old_count = guard::check(&tx, batch)?;
    let revision = batch.revision + 1;
    header::write(&tx, batch, revision, old_count)?;
    super::records::write(&tx, &batch.id, 0, &batch.messages)?;
    super::records::write(&tx, &batch.id, 1, &batch.tools)?;
    super::tool_state::apply(&tx, batch)?;
    super::events::append(&tx, batch, revision)?;
    tx.execute(
        "INSERT INTO commits VALUES (?1,?2,?3)",
        params![batch.id, revision, nonce],
    )?;
    tx.commit()?;
    Ok(revision)
}

pub(super) fn revision(path: &Path, id: &str) -> Result<i64> {
    let db = super::connection::open(path)?;
    Ok(
        db.query_row("SELECT revision FROM sessions WHERE id=?1", [id], |row| {
            row.get(0)
        })?,
    )
}
