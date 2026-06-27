//! Length-delimited framing for A2A frames over a QUIC bidirectional stream.
//!
//! QUIC gives us reliable ordered bytes per stream but no message boundaries, so
//! each [`ParsedFrame`] is serialized as a `u32` big-endian length prefix
//! followed by the SSE-style `id`/`event`/`data` block. This preserves the
//! Phase 1 wire contract (resumable `id:`) while moving it onto QUIC.

use super::super::frame::{ParsedFrame, parse_frame};

/// Serialize a frame into a length-prefixed wire block (`u32` BE + body bytes).
pub fn encode_frame(frame: &ParsedFrame) -> Vec<u8> {
    let mut body = String::new();
    if let Some(id) = &frame.id {
        body.push_str(&format!("id: {}\n", id.format()));
    }
    body.push_str(&format!("event: {}\n", frame.event));
    body.push_str(&format!("data: {}", frame.data));
    let bytes = body.into_bytes();
    let mut out = (bytes.len() as u32).to_be_bytes().to_vec();
    out.extend_from_slice(&bytes);
    out
}

/// Parse a single length-prefixed block body (the bytes after the prefix).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::stream::quic::codec::{encode_frame, decode_body};
/// use codetether_agent::a2a::stream::frame::ParsedFrame;
/// use codetether_agent::a2a::stream::event_id::EventId;
///
/// let f = ParsedFrame {
///     id: Some(EventId { epoch: "e".into(), seq: 3 }),
///     event: "progress".into(),
///     data: "{\"x\":1}".into(),
/// };
/// let wire = encode_frame(&f);
/// let body = &wire[4..];
/// let decoded = decode_body(std::str::from_utf8(body).unwrap()).unwrap();
/// assert_eq!(decoded.id.unwrap().seq, 3);
/// assert_eq!(decoded.data, "{\"x\":1}");
/// ```
pub fn decode_body(body: &str) -> Option<ParsedFrame> {
    parse_frame(body)
}
