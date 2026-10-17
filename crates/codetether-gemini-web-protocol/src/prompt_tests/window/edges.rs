use super::{Role, prompt, text};

#[test]
fn oversized_system_and_user_still_obey_hard_limit() {
    let messages = vec![
        text(Role::System, &"s".repeat(900)),
        text(Role::User, &"u".repeat(900)),
    ];
    let rendered = prompt::render_bounded(&messages, 200);
    assert!(rendered.len() <= 200);
    assert!(rendered.contains("Earlier conversation omitted"));
}

#[test]
fn all_system_history_does_not_panic_when_bounded() {
    let messages = vec![text(Role::System, &"s".repeat(900))];
    assert!(prompt::render_bounded(&messages, 100).len() <= 100);
}

#[test]
fn orphan_tool_prefix_is_dropped_before_first_user_turn() {
    let messages = vec![
        text(Role::Tool, &"orphan".repeat(200)),
        text(Role::User, "valid user turn"),
    ];
    let rendered = prompt::render_bounded(&messages, 200);
    assert!(!rendered.contains("orphan"));
    assert!(rendered.contains("valid user turn"));
}
