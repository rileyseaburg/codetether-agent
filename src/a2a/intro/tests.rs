use super::detect::{INTRO_METADATA_KEY, is_intro};
use crate::a2a::types::{Message, MessageRole, Part};

fn text_message(text: &str) -> Message {
    Message {
        message_id: "m1".to_string(),
        role: MessageRole::User,
        parts: vec![Part::Text {
            text: text.to_string(),
        }],
        context_id: None,
        task_id: None,
        metadata: std::collections::HashMap::new(),
        extensions: vec![],
    }
}

#[test]
fn detects_tagged_intro() {
    let mut msg = text_message("anything at all");
    msg.metadata.insert(
        INTRO_METADATA_KEY.to_string(),
        serde_json::Value::Bool(true),
    );
    assert!(is_intro(&msg));
}

#[test]
fn detects_legacy_intro_text() {
    let msg = text_message(
        "Hello from ubuntu-dev-riley-3fa6 (http://192.168.50.101:41219). \
         I am online and available for A2A collaboration.",
    );
    assert!(is_intro(&msg));
}

#[test]
fn real_tasks_are_not_intros() {
    assert!(!is_intro(&text_message(
        "Fix the failing test in src/lib.rs"
    )));
    assert!(!is_intro(&text_message(
        "Hello from a user who needs help with Rust"
    )));
}

#[test]
fn ledger_normalizes_trailing_slash() {
    // contains() on a never-written endpoint must be false and not panic.
    assert!(!super::ledger::contains("http://198.51.100.7:1/"));
}
