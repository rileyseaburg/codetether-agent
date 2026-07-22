//! Conversion of completed Gemini Web text into provider-neutral stream events.

use super::tool_validation;
use crate::provider::{Message, StreamChunk, ToolDefinition};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn chunks(
    text: &str,
    tools: &[ToolDefinition],
    messages: &[Message],
) -> Result<Vec<StreamChunk>> {
    let (cleaned, calls) = tool_validation::extract(text, tools, messages)?;
    Ok(from_parts(cleaned, calls))
}

pub(super) fn from_parts(cleaned: String, calls: Vec<(String, String)>) -> Vec<StreamChunk> {
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
