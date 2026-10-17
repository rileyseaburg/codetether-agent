//! AWS eventstream messages and incremental binary frame decoding.
//!
//! Feed bytes to [FrameBuffer::extend] and drain [FrameBuffer::next_frame].
//! CRC fields are retained in the framing layout but are not validated.
use anyhow::{Result, anyhow};
use std::collections::HashMap;
mod buffer;
mod headers;
use headers::parse_headers;

/// One decoded AWS eventstream message.
///
/// # Examples
/// ```
/// let message = codetether_bedrock_protocol::eventstream::EventMessage::default();
/// assert!(message.payload.is_empty());
/// assert_eq!(message.event_type(), None);
/// ```
#[derive(Debug, Clone, Default)]
pub struct EventMessage {
    /// String-typed headers only; binary types are skipped.
    pub headers: HashMap<String, String>,
    /// Raw message payload (application-defined; typically JSON).
    pub payload: Vec<u8>,
}

/// Streaming frame buffer: feed arbitrary byte chunks via [`FrameBuffer::extend`]
/// and pull out complete messages via [`FrameBuffer::next_frame`].
///
/// # Examples
/// ```
/// let mut buffer = codetether_bedrock_protocol::eventstream::FrameBuffer::new();
/// assert!(buffer.next_frame()?.is_none());
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug, Default)]
pub struct FrameBuffer {
    buf: Vec<u8>,
}
impl EventMessage {
    /// Return the `:event-type` header (event variant name).
    pub fn event_type(&self) -> Option<&str> {
        self.headers.get(":event-type").map(String::as_str)
    }

    /// Return the `:message-type` header (`event`, `exception`, `error`).
    pub fn message_type(&self) -> Option<&str> {
        self.headers.get(":message-type").map(String::as_str)
    }
}
impl FrameBuffer {
    /// Create an empty buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append more bytes from the transport.
    pub fn extend(&mut self, chunk: &[u8]) {
        self.buf.extend_from_slice(chunk);
    }
}
#[cfg(test)]
mod tests;
