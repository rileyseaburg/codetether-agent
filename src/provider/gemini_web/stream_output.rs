//! Conversion of completed Gemini Web text into provider-neutral stream events.

use super::GeminiWebProvider;
use crate::provider::StreamChunk;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn chunks(text: &str) -> Vec<StreamChunk> {
    let (cleaned, calls) = GeminiWebProvider::extract_tool_calls(text);
    let mut chunks = Vec::new();
    if !cleaned.is_empty() {
        chunks.push(StreamChunk::Text(cleaned));
    }
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    for (index, (name, arguments)) in calls.into_iter().enumerate() {
        let id = format!("gwsc_{stamp}_{index}");
        chunks.push(StreamChunk::ToolCallStart {
            id: id.clone(),
            name,
        });
        chunks.push(StreamChunk::ToolCallDelta {
            id: id.clone(),
            arguments_delta: arguments,
        });
        chunks.push(StreamChunk::ToolCallEnd { id });
    }
    chunks.push(StreamChunk::Done { usage: None });
    chunks
}

#[cfg(test)]
#[path = "stream_output_tests.rs"]
mod tests;
