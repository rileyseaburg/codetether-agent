//! Pass 1 of pairing repair: inject synthetic tool results for
//! orphan tool calls, keeping the `origins` sidecar aligned.

use std::collections::HashSet;

use crate::provider::{ContentPart, Message, Role};

use super::incremental_types::MessageOrigin;

const PLACEHOLDER: &str = "[tool result unavailable: elided by context management]";

pub(super) fn inject_synthetic(messages: &mut Vec<Message>, origins: &mut Vec<MessageOrigin>) {
    let fulfilled = collect_fulfilled_ids(messages);
    let mut to_inject: Vec<(usize, String)> = Vec::new();
    for (idx, msg) in messages.iter().enumerate() {
        if msg.role != Role::Assistant {
            continue;
        }
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part
                && !fulfilled.contains(id)
            {
                to_inject.push((idx, id.clone()));
            }
        }
    }
    for (idx, call_id) in to_inject.into_iter().rev() {
        messages.insert(idx + 1, placeholder(call_id));
        origins.insert(idx + 1, MessageOrigin::Synthetic);
    }
}

fn placeholder(tool_call_id: String) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id,
            content: PLACEHOLDER.into(),
        }],
    }
}

fn collect_fulfilled_ids(messages: &[Message]) -> HashSet<String> {
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
