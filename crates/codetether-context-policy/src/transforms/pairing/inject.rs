//! Inject missing tool results in reverse index order.
use super::*;
pub(super) fn inject(messages: &mut Vec<Message>, stats: &mut ExperimentalStats) {
    let fulfilled_ids = collect_fulfilled_ids(messages);
    let mut to_inject: Vec<(usize, String)> = Vec::new();
    for (idx, msg) in messages.iter().enumerate() {
        if msg.role != Role::Assistant {
            continue;
        }
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part
                && !fulfilled_ids.contains(id)
            {
                to_inject.push((idx, id.clone()));
            }
        }
    }
    // Inject from the end so earlier indices stay valid.
    for (idx, call_id) in to_inject.into_iter().rev() {
        let placeholder = Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: call_id,
                content: "[tool result unavailable: elided by context management]".into(),
            }],
        };
        messages.insert(idx + 1, placeholder);
        stats.snippet_hits += 1;
    }

    // Pass 2: drop tool_results that reference unknown call ids.
    // Build a prefix set of known call ids and remove any ToolResult
    // whose tool_call_id isn't in the prefix at its position.
}
