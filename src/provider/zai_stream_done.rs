//! Terminal handling for the Z.AI `data: [DONE]` sentinel.
//!
//! Three things must happen when the stream ends, in order: flush buffered
//! answer text, close any tool call left open, then decide whether the turn was
//! actually empty. Keeping them together in one place ensures a tool call is
//! never left unterminated and that the fault check sees the final output state.

use super::stream_output::OutputSeen;
use super::zai_stream_assembly::finish_stream_tool_call_chunks;
use super::zai_stream_state::ZaiStreamToolState;
use crate::provider::{StreamChunk, Usage};
use std::collections::HashMap;

/// Emits the terminal chunks for a finished Z.AI stream.
///
/// Sets `seen.text` when buffered text is flushed so the caller's fault check
/// observes it. Closing tool calls here covers the case where Z.AI ends the body
/// after the last tool delta without a `"tool_calls"` finish reason.
pub(super) fn finish(
    chunks: &mut Vec<StreamChunk>,
    text_buf: &mut String,
    seen: &mut OutputSeen,
    tool_states: &mut HashMap<usize, ZaiStreamToolState>,
    frames_seen: usize,
    last_finish: Option<&str>,
    usage: Option<Usage>,
) {
    if !text_buf.is_empty() {
        seen.text = true;
        chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
    }
    // Idempotent: only tool calls that started and have not finished emit an end.
    finish_stream_tool_call_chunks(chunks, tool_states);
    if let Some(msg) = super::capture::fault::check(*seen, frames_seen, last_finish) {
        chunks.push(StreamChunk::Error(msg));
    }
    chunks.push(StreamChunk::Done { usage });
}

#[cfg(test)]
#[path = "zai_stream_done_tests.rs"]
mod tests;
