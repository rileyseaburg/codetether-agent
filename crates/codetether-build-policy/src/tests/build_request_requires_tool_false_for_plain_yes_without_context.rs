//! Original build-policy regression case.
use crate::*;
use codetether_provider_core::{ContentPart, Message, Role};
use tempfile::tempdir;

#[test]
fn build_request_requires_tool_false_for_plain_yes_without_context() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    let session_messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "yes".to_string(),
        }],
    }];

    assert!(!build_request_requires_tool(&session_messages, root));
}
