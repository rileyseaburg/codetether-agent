//! Conversion of completed Gemini Web text into provider-neutral stream events.

use super::response_text;
#[cfg(test)]
use super::{tool_validation, usage};
#[cfg(test)]
use crate::provider::{Message, ToolDefinition};
use crate::provider::{StreamChunk, Usage};
#[cfg(test)]
use anyhow::Result;

#[path = "stream_tool_events.rs"]
mod tool_events;

#[cfg(test)]
pub(super) fn chunks(
    text: &str,
    tools: &[ToolDefinition],
    messages: &[Message],
) -> Result<Vec<StreamChunk>> {
    let (cleaned, calls) = tool_validation::extract(text, tools, messages)?;
    let usage = usage::estimate("", &cleaned, &calls);
    Ok(from_parts(cleaned, calls, usage))
}

pub(super) fn from_parts(
    cleaned: String,
    calls: Vec<(String, String)>,
    usage: Usage,
) -> Vec<StreamChunk> {
    let mut chunks = Vec::new();
    // Gemini Web has no reasoning channel, so leaked narration and chat-UI
    // affordance markup arrive inside the answer text. Reasoning is surfaced as
    // Thinking rather than dropped: it is real model output and it refreshes the
    // session watchdog. Splitting happens after tool extraction so a tool_call
    // block is never mistaken for narration.
    let visible = response_text::strip_ui_markup(&cleaned);
    let (reasoning, answer) = response_text::split_reasoning(&visible);
    if !reasoning.is_empty() {
        chunks.push(StreamChunk::Thinking(reasoning));
    }
    let answer = answer.trim();
    if !answer.is_empty() {
        chunks.push(StreamChunk::Text(answer.to_string()));
    }
    tool_events::append(&mut chunks, calls);
    chunks.push(StreamChunk::Done { usage: Some(usage) });
    chunks
}

#[cfg(test)]
#[path = "stream_output_tests.rs"]
mod tests;
