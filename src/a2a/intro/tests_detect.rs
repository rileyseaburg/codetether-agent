//! Tests for the intro-message detector.

use crate::a2a::intro::detect::{INTRO_METADATA_KEY, is_intro};
use crate::a2a::intro::tests_fixtures::text_message;

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
