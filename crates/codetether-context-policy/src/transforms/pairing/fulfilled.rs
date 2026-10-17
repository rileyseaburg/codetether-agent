//! Collect tool-result IDs for orphan call detection.
use super::*;

pub(super) fn collect_fulfilled_ids(messages: &[Message]) -> HashSet<String> {
    let mut out = HashSet::new();
    for msg in messages {
        if msg.role != Role::Tool {
            continue;
        }
        for part in &msg.content {
            if let ContentPart::ToolResult { tool_call_id, .. } = part {
                out.insert(tool_call_id.clone());
            }
        }
    }
    out
}
