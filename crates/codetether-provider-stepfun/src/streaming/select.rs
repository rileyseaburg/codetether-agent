//! Retain the existing first-event selection per HTTP chunk.
use crate::*;
pub(super) fn first(mut chunks: Vec<StreamChunk>) -> StreamChunk {
    let mut process = || -> anyhow::Result<StreamChunk> {
        if chunks.is_empty() {
            Ok(StreamChunk::Text(String::new()))
        } else if chunks.len() == 1 {
            Ok(chunks.pop().ok_or_else(|| anyhow::anyhow!("no chunks"))?)
        } else {
            // Return first chunk, others are lost (simplified)
            Ok(chunks.remove(0))
        }
    };

    match process() {
        Ok(c) => c,
        Err(e) => StreamChunk::Error(e.to_string()),
    }
}
