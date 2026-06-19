//! Per-tool-call streaming state and delta-application helpers for Z.AI.

use super::zai_stream_index::stream_tool_arguments_fragment;
use super::zai_stream_types::ZaiStreamFunction;
use crate::provider::StreamChunk;

/// Per-tool-call accumulator used while reassembling streamed tool calls.
#[derive(Debug, Default)]
pub(crate) struct ZaiStreamToolState {
    pub(crate) stream_id: String,
    pub(crate) name: Option<String>,
    pub(crate) started: bool,
    pub(crate) finished: bool,
}

/// Emit a `ToolCallStart` once per call, defaulting the name when absent.
pub(crate) fn ensure_started(
    state: &mut ZaiStreamToolState,
    chunks: &mut Vec<StreamChunk>,
    default_name: &str,
) {
    if !state.started {
        chunks.push(StreamChunk::ToolCallStart {
            id: state.stream_id.clone(),
            name: state
                .name
                .clone()
                .unwrap_or_else(|| default_name.to_string()),
        });
        state.started = true;
    }
}

/// Apply one streamed function delta (name and/or argument chunk).
pub(crate) fn apply_function(
    state: &mut ZaiStreamToolState,
    func: &ZaiStreamFunction,
    chunks: &mut Vec<StreamChunk>,
) {
    if let Some(name) = &func.name
        && !name.is_empty()
    {
        state.name = Some(name.clone());
    }
    if state.name.is_some() {
        ensure_started(state, chunks, "tool");
    }
    if let Some(arguments) = &func.arguments {
        let delta = stream_tool_arguments_fragment(arguments);
        if !delta.is_empty() {
            ensure_started(state, chunks, "tool");
            chunks.push(StreamChunk::ToolCallDelta {
                id: state.stream_id.clone(),
                arguments_delta: delta,
            });
        }
    }
}
