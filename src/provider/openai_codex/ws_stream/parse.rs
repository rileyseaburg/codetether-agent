//! Terminal-state extraction from one Responses event.

use serde_json::Value;

use super::super::{OpenAiCodexProvider, ResponsesSseParser, StreamChunk, event_error};

pub(super) struct Parsed {
    pub(super) chunks: Vec<StreamChunk>,
    pub(super) terminal: bool,
    pub(super) reusable: bool,
}

pub(super) fn event(parser: &mut ResponsesSseParser, event: &Value) -> Parsed {
    let mut chunks = Vec::new();
    if event.get("type").and_then(Value::as_str) == Some("error") && event_error::is_terminal(event)
    {
        chunks.push(StreamChunk::Error(event_error::format(
            event,
            "Realtime error",
        )));
    } else {
        OpenAiCodexProvider::parse_responses_event(parser, event, &mut chunks);
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
#[path = "parse/tests.rs"]
mod tests;
