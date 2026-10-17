//! Flush text and emit tool endings at finish events.
use crate::*;
pub(super) fn push(choice: &StreamChoice, text_buf: &mut String, chunks: &mut Vec<StreamChunk>) {
    if let Some(ref reason) = choice.finish_reason {
        if !text_buf.is_empty() {
            chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
        }
        if reason == "tool_calls"
            && let Some(tc) = choice.delta.tool_calls.as_ref().and_then(|t| t.last())
            && let Some(id) = &tc.id
        {
            chunks.push(StreamChunk::ToolCallEnd { id: id.clone() });
        }
    }
}
