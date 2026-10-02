//! Explicit copy targets and non-tool error payloads.

use super::super::{Target, latest, target};
use crate::tui::chat::message::{ChatMessage, MessageType};

#[test]
fn error_messages_are_copyable_without_chrome() {
    let messages = [ChatMessage::new(MessageType::Error, "nested diagnostic")];
    assert_eq!(latest(&messages, Target::Error), Some("nested diagnostic"));
}

#[test]
fn copy_arguments_are_explicit_and_validated() {
    for command in ["/copy", "/copy reply", "/copy tool", "/copy error"] {
        assert!(target(command).is_some());
    }
    assert!(target("/copy tools").is_none());
    assert!(target("/copy tool extra").is_none());
}
