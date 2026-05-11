//! Tests for [`super::incremental_repair::repair_with_origins`].

use crate::provider::{ContentPart, Message, Role};

use super::incremental_repair::repair_with_origins;
use super::incremental_types::MessageOrigin;

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
    assert_eq!(origins, vec![MessageOrigin::Clone(7), MessageOrigin::Synthetic]);
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
