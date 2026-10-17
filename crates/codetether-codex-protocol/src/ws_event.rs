//! Terminal-state extraction from one Responses event.

use serde_json::Value;

use crate::{CodexProtocol, ResponsesSseParser, event_error};
use codetether_provider_core::StreamChunk;

/// Chunks and transport lifecycle decisions for one decoded event.
///
/// # Examples
/// ```
/// let parsed = codetether_codex_protocol::ws_event::Parsed {
///     chunks: vec![], terminal: false, reusable: false,
/// };
/// assert!(!parsed.terminal);
/// ```
pub struct Parsed {
    /// Ordered output chunks, including keepalive for an empty event.
    pub chunks: Vec<StreamChunk>,
    /// Whether an error or Done chunk ends this response.
    pub terminal: bool,
    /// Whether Done permits reusing the connection.
    pub reusable: bool,
}

/// Decode one WebSocket event and classify stream completion.
///
/// # Arguments
/// * `parser` - State retained across events on this response.
/// * `event` - A decoded JSON event.
/// # Returns
/// Ordered chunks and terminal/reuse decisions.
/// # Examples
/// ```
/// let mut parser = codetether_codex_protocol::ResponsesSseParser::default();
/// let parsed = codetether_codex_protocol::ws_event::event(&mut parser, &serde_json::json!({}));
/// assert!(!parsed.terminal);
/// ```
pub fn event(parser: &mut ResponsesSseParser, event: &Value) -> Parsed {
    let mut chunks = Vec::new();
    if event.get("type").and_then(Value::as_str) == Some("error") && event_error::is_terminal(event)
    {
        chunks.push(StreamChunk::Error(event_error::format(
            event,
            "Realtime error",
        )));
    } else {
        CodexProtocol::parse_responses_event(parser, event, &mut chunks);
    }
    if chunks.is_empty() {
        chunks.push(StreamChunk::KeepAlive);
    }
    let failed = chunks
        .iter()
        .any(|chunk| matches!(chunk, StreamChunk::Error(_)));
    let reusable = chunks
        .iter()
        .any(|chunk| matches!(chunk, StreamChunk::Done { .. }));
    Parsed {
        chunks,
        terminal: failed || reusable,
        reusable,
    }
}

#[cfg(test)]
#[path = "ws_event/tests.rs"]
mod tests;
