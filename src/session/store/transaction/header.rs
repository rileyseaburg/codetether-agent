//! Header update and invalidation of derived ranges affected by explicit edits.
use super::super::batch::Batch;
use anyhow::Result;
use rusqlite::{Transaction, params};
pub(super) fn write(
    tx: &Transaction<'_>,
    batch: &Batch,
    revision: i64,
    old_count: usize,
) -> Result<()> {
    tx.execute(
        "INSERT INTO sessions VALUES (?1,?2,?3,?4,?5)
        ON CONFLICT(id) DO UPDATE SET revision=excluded.revision,header=excluded.header,
        message_count=excluded.message_count,tool_count=excluded.tool_count",
        params![
            batch.id,
            revision,
            batch.header,
            batch.messages.end as i64,
            batch.tools.end as i64
        ],
    )?;
    if batch.messages.start < old_count {
        tx.execute("UPDATE consumers SET revision=revision+1,
            seq=MIN(seq,(?2 / CASE name WHEN 'rlm' THEN 16 WHEN 'recall' THEN 4 WHEN 'archive' THEN 128 ELSE 1 END)
            * CASE name WHEN 'rlm' THEN 16 WHEN 'recall' THEN 4 WHEN 'archive' THEN 128 ELSE 1 END)
            WHERE session_id=?1", params![batch.id, batch.messages.start as i64])?;
        tx.execute(
            "DELETE FROM projections WHERE session_id=?1 AND end_seq>?2",
            params![batch.id, batch.messages.start as i64],
        )?;
    }
    Ok(())
}
