//! Wire-format regression for changing optional recall and prompt caching.
use crate::provider::{CompletionRequest, ContentPart, Message, Role};
use crate::session::context::request_guard::finish;

fn text(role: Role, value: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: value.into() }],
    }
}

#[test]
fn changing_recall_preserves_the_anthropic_system_and_history_prefix() {
    let request = CompletionRequest {
        messages: vec![
            text(Role::System, "stable system"),
            text(Role::User, "previous substantive user request"),
            text(Role::Assistant, "previous answer"),
            text(Role::User, "current substantive user request"),
        ],
        tools: vec![],
        model: "claude-sonnet-4-5".into(),
        temperature: None,
        top_p: None,
        max_tokens: Some(8192),
        stop: vec![],
    };
    let first = finish(request.clone(), vec![text(Role::System, "recall one")]).unwrap();
    let second = finish(request, vec![text(Role::System, "recall two")]).unwrap();
    let (first_system, first_messages) = super::super::convert::messages(&first.messages, true);
    let (second_system, second_messages) = super::super::convert::messages(&second.messages, true);
    assert_eq!(first_system, second_system);
    assert_eq!(first_messages[..2], second_messages[..2]);
    assert_ne!(first_messages[2], second_messages[2]);
    let blocks = first_system.unwrap();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0]["cache_control"]["type"], "ephemeral");
}
