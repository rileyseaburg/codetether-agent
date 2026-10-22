use super::fixtures::{request, text};
use codetether_provider_core::Role;

#[test]
fn recall_before_short_continue_does_not_move_ahead_of_prior_history() {
    let mut original = request();
    original.messages[3] = text(Role::User, "continue");
    let result = crate::finish(original.clone(), vec![text(Role::System, "new recall")]).unwrap();
    assert_eq!(
        serde_json::to_value(&result.messages[..3]).unwrap(),
        serde_json::to_value(&original.messages[..3]).unwrap()
    );
    assert_eq!(result.messages[3].role, Role::User);
    assert!(
        matches!(&result.messages[4].content[0], codetether_provider_core::ContentPart::Text{text} if text=="continue")
    );
}

#[test]
fn optional_evidence_does_not_split_a_tool_call_and_result_pair() {
    use codetether_provider_core::{ContentPart, Message};
    let mut original = request();
    original.messages.insert(
        3,
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call".into(),
                name: "read".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
        },
    );
    original.messages.insert(
        4,
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "call".into(),
                content: "result".into(),
            }],
        },
    );
    original.messages[5] = text(Role::User, "continue");
    let result = crate::finish(original.clone(), vec![text(Role::Developer, "extra")]).unwrap();
    assert_eq!(
        serde_json::to_value(&result.messages[..5]).unwrap(),
        serde_json::to_value(&original.messages[..5]).unwrap()
    );
    assert_eq!(result.messages[5].role, Role::User);
    assert_eq!(result.messages[6].role, Role::User);
}

#[test]
fn optional_evidence_follows_completed_tool_history() {
    use codetether_provider_core::{ContentPart, Message};
    let mut original = request();
    original.messages.pop();
    original.messages.push(Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "call".into(),
            name: "read".into(),
            arguments: "{}".into(),
            thought_signature: None,
        }],
    });
    original.messages.push(Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "call".into(),
            content: "result".into(),
        }],
    });
    let length = original.messages.len();
    let result = crate::finish(
        original.clone(),
        vec![text(Role::System, "changing recall")],
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(&result.messages[..length]).unwrap(),
        serde_json::to_value(&original.messages).unwrap()
    );
    assert_eq!(result.messages[length].role, Role::User);
}
