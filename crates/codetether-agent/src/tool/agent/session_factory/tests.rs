use super::create_agent_session;
use crate::provider::ContentPart;
use std::path::PathBuf;

#[tokio::test]
async fn child_session_uses_parent_workspace_not_process_cwd() {
    let parent_workspace = PathBuf::from("/tmp/codetether-parent-worktree");
    let session = create_agent_session(
        "child",
        "work in the parent repo",
        "example/model",
        Some(parent_workspace.clone()),
        true,
    )
    .await
    .unwrap();

    assert_eq!(session.metadata.directory, Some(parent_workspace.clone()));
    assert_eq!(session.metadata.inherited_prior_context_allowed, Some(true));
    assert_eq!(session.title.as_deref(), Some("Sub-agent: child"));
    let ContentPart::Text { text } = &session.messages[0].content[0] else {
        panic!("expected system prompt text");
    };
    assert!(text.contains(&parent_workspace.display().to_string()));
}
