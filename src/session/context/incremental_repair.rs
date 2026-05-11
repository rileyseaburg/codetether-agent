//! Pairing-repair pass that maintains an [`MessageOrigin`] sidecar.
//!
//! Mirrors the behaviour of
//! [`crate::session::helper::experimental::pairing::repair_orphans`]
//! but updates a parallel `origins` vector so the caller can recompute
//! `dropped_ranges` after the fact (issue #231 item 5).

use std::collections::HashSet;

use crate::provider::{ContentPart, Message, Role};

use super::incremental_types::MessageOrigin;

/// Run pairing repair in place on `messages` and keep `origins`
/// aligned. Returns `()`; the only post-condition that matters to the
/// caller is `messages.len() == origins.len()`.
///
/// # Panics
///
/// Panics in debug builds if the input lengths are out of sync.
pub fn repair_with_origins(messages: &mut Vec<Message>, origins: &mut Vec<MessageOrigin>) {
    debug_assert_eq!(messages.len(), origins.len());

    // Pass 1: inject synthetic tool results for orphan tool_calls.
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
        let placeholder = Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: call_id,
                content: "[tool result unavailable: elided by context management]".into(),
            }],
        };
        messages.insert(idx + 1, placeholder);
        origins.insert(idx + 1, MessageOrigin::Synthetic);
    }

    // Pass 2: drop tool_results that reference unknown call ids.
    let mut known: HashSet<String> = HashSet::new();
    let mut drops: Vec<usize> = Vec::new();
    for (idx, msg) in messages.iter().enumerate() {
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part {
                known.insert(id.clone());
            }
        }
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
        origins.remove(idx);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assistant_with_tool_call(id: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: id.to_string(),
                name: "noop".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
        }
    }

    #[test]
    fn injects_synthetic_for_orphan_tool_call() {
        let mut messages = vec![assistant_with_tool_call("call-1")];
        let mut origins = vec![MessageOrigin::Clone(7)];

        repair_with_origins(&mut messages, &mut origins);

        assert_eq!(messages.len(), 2);
        assert_eq!(origins.len(), 2);
        assert_eq!(origins[0], MessageOrigin::Clone(7));
        assert_eq!(origins[1], MessageOrigin::Synthetic);
    }

    #[test]
    fn drops_orphan_tool_result_and_its_origin() {
        let orphan_result = Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "missing".into(),
                content: "oops".into(),
            }],
        };
        let real = Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "hello".into(),
            }],
        };
        let mut messages = vec![orphan_result, real];
        let mut origins = vec![MessageOrigin::Clone(1), MessageOrigin::Clone(2)];

        repair_with_origins(&mut messages, &mut origins);

        assert_eq!(messages.len(), 1);
        assert_eq!(origins, vec![MessageOrigin::Clone(2)]);
    }
}
