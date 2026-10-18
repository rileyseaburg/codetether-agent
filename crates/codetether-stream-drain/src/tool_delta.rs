//! Accumulate tool-call identity and argument fragments by provider ID.
use anyhow::Result;
use codetether_stream_outcome::ToolAccumulator;
use std::collections::HashMap;

/// Handle a `ToolCallDelta` chunk: append argument bytes (size-capped via
/// [`ensure_tool_args_room`](codetether_stream_bounds::ensure_tool_args_room)).
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
        codetether_stream_bounds::ensure_tool_args_room(
            tools[idx].arguments.len(),
            arguments_delta.len(),
            &tools[idx].id,
        )?;
        tools[idx].arguments.push_str(&arguments_delta);
    } else {
        codetether_stream_bounds::ensure_tool_args_room(0, arguments_delta.len(), &id)?;
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
