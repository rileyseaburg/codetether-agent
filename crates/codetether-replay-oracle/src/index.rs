//! File-reference and tool-call ownership indexes for replay.
use codetether_context_policy::relevance::extract;
use codetether_provider_core::{ContentPart, Message};
use std::collections::HashMap;

/// Index files referenced by each turn so later turns can fast-lookup
/// "first turn this path appeared in".
pub(crate) fn index_files_per_turn(messages: &[Message]) -> Vec<Vec<String>> {
    messages.iter().map(|m| extract(m).files).collect()
}

/// Map tool_call_id → owning turn index, so a later `ToolResult` can
/// locate the `ToolCall` that introduced it.
pub(crate) fn index_tool_call_owners(messages: &[Message]) -> HashMap<String, usize> {
    let mut owners = HashMap::new();
    for (idx, msg) in messages.iter().enumerate() {
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part {
                owners.insert(id.clone(), idx);
            }
        }
    }
    owners
}
