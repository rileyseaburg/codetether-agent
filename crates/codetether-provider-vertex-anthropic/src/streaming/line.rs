//! Data-line and terminal sentinel handling.
use crate::*;
pub(super) fn line(line: &str, chunks: &mut Vec<StreamChunk>, text_buf: &mut String) {
    // Handle SSE format
    if line.starts_with("event:") {
        return; // Skip event type lines
    }

    if let Some(data) = line.strip_prefix("data: ") {
        if data == "[DONE]" {
            if !text_buf.is_empty() {
                chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
            }
            chunks.push(StreamChunk::Done { usage: None });
            return;
        }

        // Parse Anthropic streaming event
        if let Ok(event) = serde_json::from_str::<Value>(data) {
            super::event::dispatch(&event, chunks, text_buf);
        }
    }
}
