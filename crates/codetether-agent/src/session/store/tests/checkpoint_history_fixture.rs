//! Source fixture with an action and its result separated by window eviction.
use crate::provider::{ContentPart, Message, Role};
pub(super) fn history(count: usize) -> Vec<Message> {
    let mut messages = vec![
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "investigating".into()
            }]
        };
        count
    ];
    messages[0] = Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "Never deploy".into(),
        }],
    };
    messages[20] = Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "old-call".into(),
            name: "edit".into(),
            arguments: "{}".into(),
            thought_signature: None,
        }],
    };
    messages[1_100] = Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "old-call".into(),
            content: "Tool call feedback\n- status: failure\n\nOutput:\npermission denied".into(),
        }],
    };
    messages
}
