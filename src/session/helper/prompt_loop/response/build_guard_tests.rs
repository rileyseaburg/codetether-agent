use super::tool_executed;
use crate::provider::{ContentPart, Message, Role};

fn message(role: Role) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text {
            text: "content".to_string(),
        }],
    }
}

#[test]
fn tool_result_satisfies_current_user_turn() {
    let messages = [
        message(Role::User),
        message(Role::Assistant),
        message(Role::Tool),
    ];

    assert!(tool_executed(&messages));
}

#[test]
fn old_tool_result_does_not_satisfy_new_user_turn() {
    let messages = [
        message(Role::User),
        message(Role::Tool),
        message(Role::Assistant),
        message(Role::User),
    ];

    assert!(!tool_executed(&messages));
}

#[test]
fn assistant_text_alone_does_not_satisfy_turn() {
    let messages = [message(Role::User), message(Role::Assistant)];

    assert!(!tool_executed(&messages));
}
