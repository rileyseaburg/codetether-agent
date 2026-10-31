//! Sharing is the default; worktree isolation requires an explicit choice.
use codetether_swarm_tool::execution_config::ExecutionConfig;
use serde_json::json;

#[test]
fn isolation_is_an_explicit_option() {
    assert!(!ExecutionConfig::from_params(&json!({})).isolate_worktrees);
    assert!(ExecutionConfig::from_params(&json!({"isolate_worktrees": true})).isolate_worktrees);
}

#[test]
fn local_swarm_defaults_to_shared_workspace() {
    assert!(!codetether_swarm_core::SwarmConfig::default().worktree_enabled);
}
