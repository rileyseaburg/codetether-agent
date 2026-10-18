//! Original build-policy regression case.
use crate::*;
use codetether_provider_core::ToolDefinition;
use codetether_provider_core::{ContentPart, Message, Role};
use serde_json::json;
use std::fs;
use tempfile::tempdir;

#[test]
fn build_mode_tool_first_retry_does_not_trigger_for_non_build_agent() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("src/provider")).expect("mkdirs");
    fs::write(root.join("src/provider/gemini_web.rs"), "fn main(){}").expect("write file");

    let tools = vec![ToolDefinition {
        name: "bash".to_string(),
        description: "Run shell commands".to_string(),
        parameters: json!({"type":"object"}),
    }];
    let session_messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "fix src/provider/gemini_web.rs now".to_string(),
        }],
    }];

    let should_retry = should_force_build_tool_first_retry(
        "plan",
        0,
        &tools,
        &session_messages,
        root,
        "If you want, I can patch this next.",
        false,
        2,
    );

    assert!(!should_retry);
}
