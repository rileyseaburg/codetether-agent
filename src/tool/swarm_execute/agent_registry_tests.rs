use super::{standard_for_test, with_shared};
use crate::swarm::{executor::workspace_registry::Capability, result_store::ResultStore};
use crate::tool::ToolRegistry;
use std::sync::Arc;

#[test]
fn autonomous_workers_receive_real_noninteractive_tools() {
    let root = tempfile::tempdir().unwrap();
    let registry = standard_for_test(root.path(), Capability::Mutating);
    for tool in ["read", "grep", "write", "edit", "bash"] {
        assert!(registry.contains(tool), "missing executable tool: {tool}");
        assert!(registry.definitions().iter().any(|item| item.name == tool));
    }
    for tool in ["question", "swarm_execute", "agent"] {
        assert!(!registry.contains(tool), "unsafe autonomous tool: {tool}");
    }
}

#[test]
fn read_only_workers_cannot_mutate_the_workspace() {
    let root = tempfile::tempdir().unwrap();
    let registry = standard_for_test(root.path(), Capability::ReadOnly);
    for tool in ["write", "edit", "bash"] {
        assert!(!registry.contains(tool), "mutation tool exposed: {tool}");
    }
    assert!(registry.contains("read"));
}

#[tokio::test]
async fn workers_share_one_result_store() {
    let store = ResultStore::new_arc();
    let registry = |id: &str| {
        with_shared(
            ToolRegistry::with_defaults(),
            Arc::clone(&store),
            id.to_string(),
        )
    };
    let publisher = registry("publisher");
    let reader = registry("reader");
    let publish = publisher.get("swarm_share").unwrap();
    publish
        .execute(serde_json::json!({"action":"publish","key":"finding","value":42}))
        .await
        .unwrap();
    let result = reader
        .get("swarm_share")
        .unwrap()
        .execute(serde_json::json!({"action":"get","key":"finding"}))
        .await
        .unwrap();
    assert!(result.success && result.output.contains("42"));
}
