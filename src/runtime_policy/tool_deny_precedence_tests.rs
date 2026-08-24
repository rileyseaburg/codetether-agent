//! Explicit tool denial overrides specialized allow classifications.

use crate::config::{Config, PermissionAction};
use serde_json::json;

#[test]
fn tool_deny_overrides_read_only_fast_path() {
    let mut config = Config::default();
    config
        .permissions
        .tools
        .insert("bash".into(), PermissionAction::Deny);
    let args = json!({"command": "pwd"});
    let blocked = super::evaluate_tool_invocation_with_config(&config, "bash", &args)
        .expect("denied");
    assert_eq!(blocked.metadata["policy_outcome"], "deny");
}

#[test]
fn batch_deny_overrides_read_only_classification() {
    let mut config = Config::default();
    config.permissions.tools.insert("batch".into(), PermissionAction::Deny);
    let args = json!({"calls": [{"tool": "read", "args": {"path": "a"}}]});
    assert!(super::evaluate_tool_invocation_with_config(&config, "batch", &args).is_some());
}