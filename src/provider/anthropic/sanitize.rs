//! Tool-call/result pairing sanitizer for Anthropic message conversion.
//!
//! Anthropic rejects a request with `tool call result does not follow tool
//! call` (error 2013) when a `tool_result` block has no matching preceding
//! `tool_use`. This happens in practice after context compression, `/undo`,
//! or `/fork` drops the assistant turn that issued the original tool call
//! while leaving its result behind.
//!
//! This module scans the conversation once to collect every tool-call ID the
//! assistant actually declared, so the converter can drop orphan results.

use std::collections::HashSet;

use crate::provider::{ContentPart, Message, Role};

/// Collect the set of tool-call IDs declared by assistant `ToolCall` parts.
///
/// The returned set is used to filter out orphan `tool_result` blocks whose
/// originating tool call is no longer present in the conversation history.
pub(crate) fn declared_tool_call_ids(messages: &[Message]) -> HashSet<String> {
    let mut ids = HashSet::new();
    for msg in messages {
        if msg.role != Role::Assistant {
            continue;
        }
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part {
                ids.insert(id.clone());
            }
        }
    }
    ids
}

/// Whether a tool-result content part references a known tool-call ID.
///
/// Non-tool-result parts always pass through (`true`) so only orphan results
/// are filtered.
pub(crate) fn result_has_call(part: &ContentPart, known: &HashSet<String>) -> bool {
    match part {
        ContentPart::ToolResult { tool_call_id, .. } => known.contains(tool_call_id),
        _ => true,
    }
}
