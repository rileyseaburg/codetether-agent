//! Tool-call accumulation helpers for the stream collector.
//!
//! Tracks tool calls by id as `ToolCallStart` / `ToolCallDelta` chunks
//! arrive, including the out-of-order case where argument deltas show up
//! before (or without) a start chunk.

use super::finalize::ToolAccumulator;
use anyhow::Result;
use std::collections::HashMap;

/// Handle a `ToolCallStart` chunk: open a new accumulator, or backfill the
/// name when deltas arrived first under the placeholder name `"tool"`.
pub(super) fn on_tool_start(
    tools: &mut Vec<ToolAccumulator>,
    tool_index_by_id: &mut HashMap<String, usize>,
    id: String,
    name: String,
) {
    let next_idx = tools.len();
    let idx = *tool_index_by_id.entry(id.clone()).or_insert(next_idx);
    if idx == next_idx {
        tools.push(ToolAccumulator {
            id,
            name,
            arguments: String::new(),
        });
    } else if tools[idx].name == "tool" {
        tools[idx].name = name;
    }
}

/// Handle a `ToolCallDelta` chunk: append argument bytes (size-capped via
/// [`ensure_tool_args_room`](crate::session::helper::stream_caps::ensure_tool_args_room)).
///
/// # Errors
///
/// Returns an error when the accumulated arguments would exceed the cap.
pub(super) fn on_tool_delta(
    tools: &mut Vec<ToolAccumulator>,
    tool_index_by_id: &mut HashMap<String, usize>,
    id: String,
    arguments_delta: String,
) -> Result<()> {
    if let Some(idx) = tool_index_by_id.get(&id).copied() {
        super::super::stream_caps::ensure_tool_args_room(
            tools[idx].arguments.len(),
            arguments_delta.len(),
            &tools[idx].id,
        )?;
        tools[idx].arguments.push_str(&arguments_delta);
    } else {
        super::super::stream_caps::ensure_tool_args_room(0, arguments_delta.len(), &id)?;
        let idx = tools.len();
        tool_index_by_id.insert(id.clone(), idx);
        tools.push(ToolAccumulator {
            id,
            name: "tool".to_string(),
            arguments: arguments_delta,
        });
    }
    Ok(())
}
