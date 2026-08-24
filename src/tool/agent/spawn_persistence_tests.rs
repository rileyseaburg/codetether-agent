//! Durable child-session failure regression.

use super::MockProvider;
use crate::tool::{Tool, agent::tool_impl::AgentTool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn durable_spawn_fails_when_session_store_is_unwritable() {
    let (dir, _guard) = super::super::persistence::test_support::isolate();
    std::fs::write(dir.path().join("sessions"), "not a directory").expect("blocker");
    let mut registry = crate::provider::ProviderRegistry::new();
    registry.register(Arc::new(MockProvider));
    super::super::registry::set_registry_for_test(Arc::new(registry)).await;

    let mut args = json!({
        "action": "spawn", "name": "persist_fail",
        "instructions": "test", "model": "mock/paid:free",
        "fork_turns": "none",
        "__ct_session_id": "spawn-persistence-test"
    });
    crate::tool::network_access::bind_trusted(&mut args, true);
    let result = AgentTool::new().execute(args).await.expect("tool result");

    assert!(!result.success);
    assert!(
        result.output.contains("child session persistence failed"),
        "{}",
        result.output,
    );
}
