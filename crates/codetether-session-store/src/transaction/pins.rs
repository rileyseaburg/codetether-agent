//! Sparse page edits share the normal header revision, audit log and transaction.
use crate::{Batch, pin_edits::PinSource};
use anyhow::{Result, ensure};
use rusqlite::{Transaction, params};
use sha2::{Digest, Sha256};

pub(super) fn nonce(batch: &Batch, edits: &[PinSource]) -> Result<String> {
    let base = crate::identity::nonce(batch);
    if edits.is_empty() {
        return Ok(base);
    }
    let mut hash = Sha256::new();
    hash.update(b"indexed-pin-edit-v1");
    hash.update(base.as_bytes());
    for edit in edits {
        hash.update(serde_json::to_vec(&(
            edit.index,
            &edit.message,
            edit.original,
            edit.page,
        ))?);
    }
    Ok(hex::encode(hash.finalize()))
}

pub(super) fn write(
    tx: &Transaction<'_>,
    batch: &Batch,
    revision: i64,
    edits: &[PinSource],
) -> Result<()> {
    ensure!(
        edits.len() <= 2,
        "at most two indexed page edits may be committed"
    );
    for edit in edits {
        ensure!(
            edit.index < batch.messages.start,
            "indexed pin edit overlaps the regular message suffix"
        );
        let rows = crate::read::read_window(tx, &batch.id, 0, edit.index, edit.index + 1)?;
        ensure!(rows.len() == 1, "indexed pin source is missing");
        let actual: (
            codetether_provider_core::Message,
            codetether_context_policy::pages::PageKind,
        ) = serde_json::from_str(&rows[0])?;
        ensure!(
            serde_json::to_value(actual)? == serde_json::to_value((&edit.message, edit.original))?,
            "SESSION_REVISION_CONFLICT: indexed pin source changed"
        );
        let body = crate::blobs::put(tx, &serde_json::to_string(&(&edit.message, edit.page))?)?;
        tx.execute(
            "UPDATE records SET body=?3 WHERE session_id=?1 AND kind=0 AND seq=?2",
            params![batch.id, edit.index as i64, body],
        )?;
        tx.execute(
            "DELETE FROM constraints WHERE session_id=?1 AND seq=?2",
            params![batch.id, edit.index as i64],
        )?;
        crate::constraints::record(tx, &batch.id, edit.index as i64, &edit.message, edit.page)?;
        tx.execute(
            "INSERT INTO events VALUES (?1,?2,0,?3,'put',?4)",
            params![batch.id, revision, edit.index as i64, body],
        )?;
    }
    Ok(())
}
