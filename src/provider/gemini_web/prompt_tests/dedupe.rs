use super::{ContentPart, Message, Role, prompt};

fn text(role: Role, value: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: value.into() }],
    }
}

#[test]
fn keeps_only_latest_byte_identical_system_message() {
    let messages = vec![
        text(Role::System, "same"),
        text(Role::User, "first"),
        text(Role::System, "same"),
        text(Role::User, "second"),
    ];
    let rendered = prompt::render_bounded(&messages, 2_000);
    assert_eq!(rendered.matches("System: same").count(), 1);
    assert!(rendered.contains("User: first"));
    assert!(rendered.contains("User: second"));
}

#[test]
fn retains_distinct_system_messages() {
    let messages = vec![text(Role::System, "first"), text(Role::System, "second")];
    let rendered = prompt::render_bounded(&messages, 2_000);
    assert!(rendered.contains("System: first"));
    assert!(rendered.contains("System: second"));
}
