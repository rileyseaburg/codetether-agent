use codetether_provider_core::{ContentPart, Message, Role, runtime_context};
fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
fn encode(state: &str) -> serde_json::Value {
    let messages = codetether_context_policy::request::messages::build(
        &runtime_context::append("stable rules", state),
        vec![
            text(Role::User, "original task"),
            text(Role::Assistant, "prior result"),
            text(Role::User, "continue"),
        ],
    );
    serde_json::to_value(super::convert(&messages).unwrap()).unwrap()
}
#[test]
fn runtime_goal_state_does_not_change_openai_chat_system_or_prefix() {
    let first = encode("used: 0");
    let second = encode("used: 750; constraint: do not deploy");
    assert_eq!(
        &first.as_array().unwrap()[..3],
        &second.as_array().unwrap()[..3]
    );
    assert_eq!(second[0]["role"], "system");
    assert_eq!(second[0]["content"], "stable rules");
    assert_eq!(second[3]["role"], "user");
    assert!(second[3].to_string().contains("750"));
    assert_eq!(second[4]["content"], "continue");
}
