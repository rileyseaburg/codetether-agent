//! Durable child creation routes both persisted cwd and prompts to its checkout.

use crate::provider::ContentPart;
use crate::session::Session;
use crate::tool::agent::{params::Params, persistence, spawn_request::SpawnRequest};
use serde_json::json;

#[path = "workspace/test_support.rs"]
mod support;

#[tokio::test]
async fn child_session_pins_and_persists_its_managed_checkout() {
    let (_data, _guard) = persistence::test_support::isolate();
    let repo = support::fixture();
    let params: Params = serde_json::from_value(json!({
        "action": "spawn", "name": "isolated-worker", "model": "test/model",
        "instructions": "Edit project/data.txt", "fork_turns": "none",
        "__ct_parent_workspace": repo.path(), "__ct_prior_context_allowed": false
    }))
    .unwrap();
    let request = SpawnRequest::from_params(&params).unwrap();
    let (session, handoff) = super::create(&params, &request).await.unwrap();
    assert_eq!(
        session.metadata.directory.as_ref(),
        Some(&handoff.workspace)
    );
    assert!(session.metadata.workspace_pinned);
    assert_eq!(handoff.base_commit, support::head(repo.path()));
    let ContentPart::Text { text } = &session.messages[0].content[0] else {
        panic!("expected child system prompt")
    };
    assert!(text.contains(&format!("Workspace cwd: {}", handoff.workspace.display())));
    let ContentPart::Text { text } = &session.messages.last().unwrap().content[0] else {
        panic!("expected isolation guidance")
    };
    assert!(text.contains("Parent uncommitted changes are NOT copied"));
    session.save().await.unwrap();
    let loaded = Session::load(&session.id).await.unwrap();
    assert!(loaded.metadata.workspace_pinned);
    assert_eq!(loaded.metadata.directory, Some(handoff.workspace.clone()));
    support::commit_data(&handoff.worktree, "child change\n");
    assert_eq!(support::head(repo.path()), handoff.base_commit);
    assert_eq!(
        std::fs::read_to_string(repo.path().join("project/data.txt")).unwrap(),
        "primary\n"
    );
    assert_eq!(loaded.metadata.inherited_prior_context_allowed, Some(false));
}
