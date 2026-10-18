//! Original build-policy regression case.
use crate::*;
use codetether_provider_core::{ContentPart, Message, Role};
use std::fs;
use tempfile::tempdir;

#[test]
fn build_request_requires_tool_for_affirmative_followup_with_context() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::write(root.join("rspack.config.js"), "module.exports = {};").expect("write file");

    let session_messages = vec![
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "ws error in rspack.config.js".to_string(),
            }],
        },
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "Done — use this exact block in rspack.config.js".to_string(),
            }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "yes".to_string(),
            }],
        },
    ];

    assert!(build_request_requires_tool(&session_messages, root));
}
