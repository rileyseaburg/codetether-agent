//! Assembly of Z.AI streamed tool-call deltas into ordered [`StreamChunk`]s.
//!
//! The Z.AI SSE stream delivers tool calls as a sequence of partial deltas
//! that must be reassembled by index. These entry points own that reassembly.

use super::zai_stream_index::resolve_tool_index;
use super::zai_stream_state::{ZaiStreamToolState, apply_function};
use super::zai_stream_types::ZaiStreamToolCall;
use crate::provider::StreamChunk;
use std::collections::HashMap;

/// Reassemble streamed tool-call deltas into ordered [`StreamChunk`] events.
pub(crate) fn append_stream_tool_call_chunks(
    chunks: &mut Vec<StreamChunk>,
    tool_calls: &[ZaiStreamToolCall],
    tool_states: &mut HashMap<usize, ZaiStreamToolState>,
    next_synthetic_index: &mut usize,
    last_seen_index: &mut Option<usize>,
) {
    for tc in tool_calls {
        let index = resolve_tool_index(tc, tool_states, next_synthetic_index, last_seen_index);
        let state = tool_states
            .entry(index)
            .or_insert_with(|| ZaiStreamToolState {
                stream_id: tc.id.clone().unwrap_or_else(|| format!("zai-tool-{index}")),
                ..Default::default()
            });
        if let Some(id) = &tc.id
            && !state.started
            && state.stream_id.starts_with("zai-tool-")
        {
            state.stream_id = id.clone();
        }
        if let Some(func) = &tc.function {
            apply_function(state, func, chunks);
        }
    }
}

/// Emit `ToolCallEnd` events for every started-but-unfinished tool call.
pub(crate) fn finish_stream_tool_call_chunks(
    chunks: &mut Vec<StreamChunk>,
    tool_states: &mut HashMap<usize, ZaiStreamToolState>,
) {
    let mut ordered_indexes: Vec<_> = tool_states.keys().copied().collect();
    ordered_indexes.sort_unstable();
    for index in ordered_indexes {
        if let Some(state) = tool_states.get_mut(&index)
            && state.started
            && !state.finished
        {
            chunks.push(StreamChunk::ToolCallEnd {
                id: state.stream_id.clone(),
            });
            state.finished = true;
        }
    }
}
