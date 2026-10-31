//! Apply one revision-checked batch on the coordinator-owned connection.
use super::{guard, header, pins};
use crate::{Batch, pin_edits::PinSource};
use anyhow::Result;
use rusqlite::{Connection, TransactionBehavior, params};

pub(crate) fn run(db: &mut Connection, batch: &Batch, edits: &[PinSource]) -> Result<i64> {
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let nonce = pins::nonce(batch, edits)?;
    if let Some(revision) = guard::retry(&tx, &batch.id, &nonce)? {
        return Ok(revision);
    }
    let old_count = guard::check(&tx, batch)?;
    let revision = batch.revision + 1;
    let start = edits
        .iter()
        .map(|edit| edit.index)
        .min()
        .unwrap_or(batch.messages.start)
        .min(batch.messages.start);
    header::write(&tx, batch, revision, old_count, start)?;
    crate::records::write(&tx, &batch.id, 0, &batch.messages)?;
    crate::records::write(&tx, &batch.id, 1, &batch.tools)?;
    crate::tool_state::apply(&tx, batch)?;
    crate::events::append(&tx, batch, revision)?;
    pins::write(&tx, batch, revision, edits)?;
    tx.execute(
        "INSERT INTO commits VALUES (?1,?2,?3)",
        params![batch.id, revision, nonce],
    )?;
    tx.commit()?;
    Ok(revision)
}
