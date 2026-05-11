use super::{build_system_message, create_agent_session};
use crate::provider::ContentPart;
use std::path::PathBuf;

#[test]
fn includes_agent_name_and_instructions() {
    let msg = build_system_message("reviewer", "Audit the PR", None);
    assert!(msg.contains("@reviewer"));
    assert!(msg.contains("Audit the PR"));
}

#[test]
fn embeds_current_working_directory() {
    let msg = build_system_message("x", "do the thing", None);
    let cwd = std::env::current_dir().unwrap().display().to_string();
    assert!(
        msg.contains(&cwd),
        "system prompt should embed cwd: {cwd}\nmsg: {msg}"
    );
}

#[test]
fn warns_against_discovery_tool_calls() {
    let msg = build_system_message("x", "y", None);
    assert!(msg.contains("Do NOT waste turns discovering the workspace"));
    assert!(msg.contains("no pwd/ls/glob"));
}

#[tokio::test]
async fn child_session_uses_parent_workspace_not_process_cwd() {
    let parent_workspace = PathBuf::from("/tmp/codetether-parent-worktree");
    let session = create_agent_session(
        "child",
        "work in the parent repo",
        "example/model",
        Some(parent_workspace.clone()),
    )
    .await
    .unwrap();

    assert_eq!(session.metadata.directory, Some(parent_workspace.clone()));
    let ContentPart::Text { text } = &session.messages[0].content[0] else {
        panic!("expected system prompt text");
    };
    assert!(text.contains(&parent_workspace.display().to_string()));
}
