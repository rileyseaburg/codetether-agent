//! Durable spawn persistence failures use disposable managed Git storage.

use super::tool_impl::AgentTool;
use crate::tool::Tool;
use serde_json::json;
use std::sync::Arc;

#[path = "spawn_git_fixture.rs"]
mod git;
#[path = "spawn_mock_provider.rs"]
mod provider;
use provider::MockProvider;

#[tokio::test]
async fn durable_spawn_fails_when_session_store_is_unwritable() {
    let (dir, _guard) = super::persistence::test_support::isolate();
    let repo = git::fixture();
    std::fs::write(dir.path().join("sessions"), "not a directory").expect("blocker");
    let mut registry = crate::provider::ProviderRegistry::new();
    registry.register(Arc::new(MockProvider));
    super::registry::set_registry_for_test(Arc::new(registry)).await;

    let result = AgentTool::new()
        .execute(json!({
            "action": "spawn", "name": "persist_fail",
            "instructions": "test", "model": "mock/paid:free",
            "__ct_parent_workspace": repo.path(), "__ct_prior_context_allowed": false
        }))
        .await
        .expect("tool result");

    assert!(!result.success);
    assert!(result.output.contains("child session persistence failed"));
    let storage = repo.path().join(".codetether-worktrees");
    let children: Vec<_> = std::fs::read_dir(&storage).unwrap().collect();
    assert_eq!(children.len(), 1);
    assert!(children[0].as_ref().unwrap().path().join(".git").is_file());
}
