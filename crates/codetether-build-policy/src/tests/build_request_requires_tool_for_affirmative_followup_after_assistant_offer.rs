//! Original build-policy regression case.
use crate::*;
use codetether_provider_core::{ContentPart, Message, Role};
use tempfile::tempdir;

#[test]
fn build_request_requires_tool_for_affirmative_followup_after_assistant_offer() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    let session_messages = vec![
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "the banner is missing".to_string(),
            }],
        },
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "If you want, next I can tighten the Catalyst alert variant exactly."
                    .to_string(),
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
