//! Remove orphan tool-result messages in reverse index order.
use super::*;
pub(super) fn drop_results(messages: &mut Vec<Message>, stats: &mut ExperimentalStats) {
    let mut known: HashSet<String> = HashSet::new();
    let mut drops: Vec<usize> = Vec::new();
    for (idx, msg) in messages.iter().enumerate() {
        // Record tool_call ids from this message for later results.
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part {
                known.insert(id.clone());
            }
        }
        // If this message is tool-role with *every* result orphaned,
        // mark for drop.
        if msg.role == Role::Tool
            && !msg.content.is_empty()
            && msg.content.iter().all(|p| match p {
                ContentPart::ToolResult { tool_call_id, .. } => !known.contains(tool_call_id),
                _ => false,
            })
        {
            drops.push(idx);
        }
    }
    for idx in drops.into_iter().rev() {
        messages.remove(idx);
        stats.snippet_hits += 1;
    }
}
