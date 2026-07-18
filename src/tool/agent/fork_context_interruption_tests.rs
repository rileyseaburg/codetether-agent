use super::{ForkTurns, select};
use crate::provider::{ContentPart, Message, Role};

fn text(role: Role, value: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: value.into() }],
    }
}

#[test]
fn interrupted_fork_keeps_marker_but_drops_assistant_tail() {
    let source = vec![
        text(Role::User, "run command"),
        text(Role::Assistant, "partial output"),
        text(Role::Developer, "interrupted"),
    ];
    let inherited = select(&source, ForkTurns::All);
    assert_eq!(inherited.len(), 2);
    assert_eq!(inherited[0].role, Role::User);
    assert_eq!(inherited[1].role, Role::Developer);
}
