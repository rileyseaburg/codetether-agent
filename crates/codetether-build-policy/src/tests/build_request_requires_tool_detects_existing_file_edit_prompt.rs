//! Original build-policy regression case.
use crate::*;
use codetether_provider_core::{ContentPart, Message, Role};
use std::fs;
use tempfile::tempdir;

#[test]
fn build_request_requires_tool_detects_existing_file_edit_prompt() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("src/provider")).expect("mkdirs");
    fs::write(root.join("src/provider/gemini_web.rs"), "fn main(){}").expect("write file");

    let session_messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "In build mode, make a concrete change now: in src/provider/gemini_web.rs replace \"A\" with \"B\".".to_string(),
        }],
    }];

    assert!(build_request_requires_tool(&session_messages, root));
}
