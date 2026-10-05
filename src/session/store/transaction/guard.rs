//! Optimistic concurrency and idempotent transaction acknowledgement checks.
use super::super::batch::Batch;
use anyhow::Result;
use rusqlite::{OptionalExtension, Transaction};
pub(super) fn retry(tx: &Transaction<'_>, id: &str, nonce: &str) -> Result<Option<i64>> {
    let committed = tx
        .query_row(
            "SELECT revision FROM commits WHERE nonce=?1",
            [nonce],
            |r| r.get::<_, i64>(0),
        )
        .optional()?;
    if let Some(revision) = committed {
        let actual: i64 = tx.query_row("SELECT revision FROM sessions WHERE id=?1", [id], |r| {
            r.get(0)
        })?;
        anyhow::ensure!(
            actual == revision,
            "SESSION_REVISION_CONFLICT: retry superseded by another writer"
        );
    }
    Ok(committed)
}
pub(super) fn check(tx: &Transaction<'_>, batch: &Batch) -> Result<usize> {
    let current = tx
        .query_row(
            "SELECT revision,message_count FROM sessions WHERE id=?1",
            [&batch.id],
            |r| Ok((r.get::<_, i64>(0)?, r.get::<_, usize>(1)?)),
        )
        .optional()?
        .unwrap_or_default();
    anyhow::ensure!(
        current.0 == batch.revision,
        "SESSION_REVISION_CONFLICT: reload session {} before writing",
        batch.id
    );
    Ok(current.1)
}
