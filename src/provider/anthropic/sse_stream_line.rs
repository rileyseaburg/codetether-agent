//! Decode SSE lines and dispatch content and usage events.

use serde_json::Value;

use super::{super::sse_line, SseChunkStream};
use crate::provider::StreamChunk;

impl SseChunkStream {
    /// Process one SSE line and return at most one chunk.
    pub(crate) fn process_line(&mut self, line: &str) -> Option<StreamChunk> {
        let (event_type, data) = sse_line::parse_sse_line(line)?;
        if let Some(ev) = event_type {
            self.pending_event = Some(ev);
            return None;
        }
        let data_str = data?;
        if data_str == "[DONE]" {
            return Some(StreamChunk::Done { usage: None });
        }
        let event: Value = serde_json::from_str(&data_str).ok()?;
        match event.get("type")?.as_str()? {
            "content_block_start" => self.blocks.start(&event),
            "content_block_delta" => self.blocks.delta(&event),
            "message_start" | "message_delta" => {
                super::super::sse_message_delta::parse(&event, &mut self.usage)
            }
            _ => None,
        }
    }
}
