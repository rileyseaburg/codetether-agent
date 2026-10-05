//! Stateful SSE byte-stream to [`StreamChunk`](crate::provider::StreamChunk) converter.
//!
//! [`SseChunkStream`] holds buffering state needed to convert a raw SSE
//! byte stream into provider-neutral chunks.
//! The [`Stream`](futures::Stream) impl is in the sibling polling module.

use bytes::Bytes;

use crate::provider::Usage;

use super::sse_block_parser::BlockParser;
#[path = "sse_stream_line.rs"]
mod line;

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
    /// Usage accumulated from message-start and message-delta events.
    usage: Option<Usage>,
    /// Set to `true` once a real [`Done`](crate::provider::StreamChunk::Done) has been yielded from
    /// a `message_delta` SSE event. Used by the poll impl to distinguish a
    /// clean byte-stream close (after a real Done) from a premature EOF.
    pub(crate) saw_done: bool,
    /// Prevents repeated premature-EOF errors after the inner stream closes.
    pub(crate) eof_reported: bool,
}

impl SseChunkStream {
    /// Create a new stream converter wrapping an HTTP response body.
    pub(crate) fn new(resp: reqwest::Response) -> Self {
        Self {
            inner: Box::pin(resp.bytes_stream()),
            buffer: String::new(),
            pending_event: None,
            blocks: BlockParser::new(),
            usage: None,
            saw_done: false,
            eof_reported: false,
        }
    }
}

#[cfg(test)]
#[path = "sse_stream_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "sse_stream_premature_eof_tests.rs"]
mod premature_eof_tests;

#[cfg(test)]
#[path = "sse_usage_tests.rs"]
mod usage_tests;
