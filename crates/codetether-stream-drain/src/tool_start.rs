//! Accumulate tool-call identity and argument fragments by provider ID.
use codetether_stream_outcome::ToolAccumulator;
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
