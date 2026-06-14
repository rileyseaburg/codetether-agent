//! Stateful SSE byte-stream to [`StreamChunk`] converter.
//!
//! [`SseChunkStream`] holds buffering state needed to convert a raw SSE
//! byte stream into provider-neutral [`StreamChunk`] values.
//! The [`Stream`] impl is in [`sse_stream_poll`].

use bytes::Bytes;
use serde_json::Value;

use crate::provider::StreamChunk;

use super::sse_block_parser::BlockParser;
use super::sse_line;

/// Stateful converter from SSE HTTP bytes to provider stream chunks.
pub(crate) struct SseChunkStream {
    /// Inner byte stream from the HTTP response body.
    pub(crate) inner:
        std::pin::Pin<Box<dyn futures::Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    /// Line-oriented text buffer for partial SSE reads.
    pub(crate) buffer: String,
    /// Pending event type accumulated until data arrives.
    pub(crate) pending_event: Option<String>,
    /// Content-block parser with tool-call ID tracking.
    blocks: BlockParser,
}

impl SseChunkStream {
    /// Create a new stream converter wrapping an HTTP response body.
    pub(crate) fn new(resp: reqwest::Response) -> Self {
        Self {
            inner: Box::pin(resp.bytes_stream()),
            buffer: String::new(),
            pending_event: None,
            blocks: BlockParser::new(),
        }
    }

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
            "message_delta" => Some(super::sse_message_delta::parse(&event)),
            _ => None,
        }
    }
}
