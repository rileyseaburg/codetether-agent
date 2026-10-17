//! Decode a single data line without accumulating across HTTP chunks.
use crate::*;
pub(super) fn push(line: &str, chunks: &mut Vec<StreamChunk>) {
    if let Some(data) = line.strip_prefix("data: ") {
        if data.trim() == "[DONE]" {
            chunks.push(StreamChunk::Done { usage: None });
            return;
        }

        if let Ok(chunk) = serde_json::from_str::<StreamChunkResponse>(data)
            && let Some(choice) = chunk.choices.first()
        {
            super::choice::push(choice, chunks);
        }
    }
}
