//! Existing page policy regression.
use super::*;

#[test]
fn classify_defaults_user_and_assistant_to_conversation() {
    assert_eq!(classify(&user("hi")), PageKind::Conversation);
    assert_eq!(classify(&assistant("reply")), PageKind::Conversation);
}
