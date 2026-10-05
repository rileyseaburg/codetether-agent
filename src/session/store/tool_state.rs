//! Tool outcomes are unknown until their actual result is committed; never replay effects.
use crate::provider::{ContentPart, Message};
use crate::session::pages::PageKind;
use anyhow::Result;
use rusqlite::{Transaction, params};
pub(super) fn apply(tx: &Transaction<'_>, batch: &super::batch::Batch) -> Result<()> {
    let start = batch.messages.start as i64;
    tx.execute(
        "DELETE FROM tool_calls WHERE session_id=?1 AND seq>=?2",
        params![batch.id, start],
    )?;
    tx.execute(
        "DELETE FROM constraints WHERE session_id=?1 AND seq>=?2",
        params![batch.id, start],
    )?;
    tx.execute("UPDATE tool_calls SET state='unknown',result_seq=NULL WHERE session_id=?1 AND result_seq>=?2", params![batch.id, start])?;
    for (index, body) in batch.messages.rows.iter().enumerate() {
        let (message, page): (Message, PageKind) = serde_json::from_str(body)?;
        let seq = (batch.messages.start + index) as i64;
        super::constraints::record(tx, &batch.id, seq, &message, page)?;
        for part in message.content {
            match part {
                ContentPart::ToolCall { id, .. } => {
                    tx.execute(
                        "INSERT INTO tool_calls VALUES (?1,?2,'unknown',?3,NULL)
                        ON CONFLICT(session_id,call_id) DO NOTHING",
                        params![batch.id, id, seq],
                    )?;
                }
                ContentPart::ToolResult { tool_call_id, .. } => {
                    tx.execute(
                        "UPDATE tool_calls SET state='result_recorded',result_seq=?3
                        WHERE session_id=?1 AND call_id=?2",
                        params![batch.id, tool_call_id, seq],
                    )?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
