//! does not split tool call pair.
use super::*;

#[test]
pub(super) fn does_not_split_tool_call_pair() {
    // Build a history that would put a ToolCall at cut_end-1 if we
    // weren't careful. Assert it moves the cut to protect the pair.
    let mut msgs = vec![u("sink0"), a("sink1")];
    for _ in 0..40 {
        msgs.push(u("filler"));
        msgs.push(a("filler"));
    }
    // Inject an open tool call right at the boundary.
    let boundary = msgs.len() - RECENT_MESSAGES - 1;
    msgs[boundary] = Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "call_1".into(),
            name: "read_file".into(),
            arguments: "{}".into(),
            thought_signature: None,
        }],
    };
    let _ = trim_middle(&mut msgs);
    // Walk: there must not be a ToolCall whose next message isn't a Tool.
    for i in 0..msgs.len().saturating_sub(1) {
        if has_open_tool_call(&msgs[i]) {
            // Next message must be Role::Tool or still Assistant (multi-call).
            let next = &msgs[i + 1];
            assert!(
                next.role == Role::Tool || next.role == Role::Assistant,
                "tool call at {i} orphaned; next role {:?}",
                next.role
            );
        }
    }
}
