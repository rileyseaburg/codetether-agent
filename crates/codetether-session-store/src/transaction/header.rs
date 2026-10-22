//! Header update and invalidation of derived ranges affected by explicit edits.
use super::super::batch::Batch;
use anyhow::Result;
use rusqlite::{Transaction, params};
pub fn write(
    tx: &Transaction<'_>,
    batch: &Batch,
    revision: i64,
    old_count: usize,
    changed_start: usize,
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
    if changed_start < old_count {
        tx.execute("UPDATE consumers SET revision=revision+1,
            seq=MIN(seq,(?2 / CASE WHEN name='rlm' OR name GLOB 'rlm:*' THEN 16 WHEN name='recall' THEN 4 WHEN name='archive' THEN 128 ELSE 1 END)
            * CASE WHEN name='rlm' OR name GLOB 'rlm:*' THEN 16 WHEN name='recall' THEN 4 WHEN name='archive' THEN 128 ELSE 1 END)
            WHERE session_id=?1", params![batch.id, changed_start as i64])?;
        tx.execute(
            "DELETE FROM projections WHERE session_id=?1 AND end_seq>?2",
            params![batch.id, changed_start as i64],
        )?;
    }
    Ok(())
}
