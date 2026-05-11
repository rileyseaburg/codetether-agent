//! Pass 2 of pairing repair: drop tool results that reference an
//! unknown call id, removing the matching origin in lock-step.

use std::collections::HashSet;

use crate::provider::{ContentPart, Message, Role};

use super::incremental_types::MessageOrigin;

pub(super) fn drop_orphan_results(messages: &mut Vec<Message>, origins: &mut Vec<MessageOrigin>) {
    let mut known: HashSet<String> = HashSet::new();
    let mut drops: Vec<usize> = Vec::new();
    for (idx, msg) in messages.iter().enumerate() {
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part {
                known.insert(id.clone());
            }
        }
        if msg.role == Role::Tool && all_results_orphan(msg, &known) {
            drops.push(idx);
        }
    }
    for idx in drops.into_iter().rev() {
        messages.remove(idx);
        origins.remove(idx);
    }
}

fn all_results_orphan(msg: &Message, known: &HashSet<String>) -> bool {
    !msg.content.is_empty()
        && msg.content.iter().all(|p| match p {
            ContentPart::ToolResult { tool_call_id, .. } => !known.contains(tool_call_id),
            _ => false,
        })
}
