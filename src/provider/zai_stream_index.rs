//! Index resolution and argument-fragment helpers for Z.AI tool streaming.

use super::zai_stream_state::ZaiStreamToolState;
use super::zai_stream_types::ZaiStreamToolCall;
use serde_json::Value;
use std::collections::HashMap;

/// Normalize a streamed tool-call `arguments` value into a string fragment.
pub(crate) fn stream_tool_arguments_fragment(arguments: &Value) -> String {
    match arguments {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        other => serde_json::to_string(other).unwrap_or_default(),
    }
}

/// Resolve the stable index for a streamed tool-call fragment, synthesizing a
/// new index when the provider omits one and no prior call can be matched.
pub(crate) fn resolve_tool_index(
    tc: &ZaiStreamToolCall,
    tool_states: &HashMap<usize, ZaiStreamToolState>,
    next_synthetic_index: &mut usize,
    last_seen_index: &mut Option<usize>,
) -> usize {
    let index = tc
        .index
        .or_else(|| {
            tc.id.as_ref().and_then(|id| {
                tool_states
                    .iter()
                    .find_map(|(idx, state)| (state.stream_id == *id).then_some(*idx))
            })
        })
        .or(*last_seen_index)
        .unwrap_or_else(|| {
            let idx = *next_synthetic_index;
            *next_synthetic_index += 1;
            idx
        });
    *last_seen_index = Some(index);
    index
}
