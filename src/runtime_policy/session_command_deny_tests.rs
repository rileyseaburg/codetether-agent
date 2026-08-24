use crate::approval::{session_command_grants, test_env::lock_env};
use crate::config::{Config, PermissionAction};
use serde_json::json;

struct GrantGuard;

impl Drop for GrantGuard {
    fn drop(&mut self) {
        session_command_grants::reset();
    }
}

#[test]
fn configured_deny_overrides_session_prefix() {
    let _lock = lock_env();
    session_command_grants::reset();
    let _guard = GrantGuard;
    let workspace = std::env::current_dir().expect("workspace");
    let workspace = workspace.to_str().expect("workspace path");
    session_command_grants::remember_scoped_request_in(
        "approval-1",
        vec!["cargo test".into()],
        Some("deny-test"),
        Some(workspace),
    );
    session_command_grants::grant_for_request("approval-1");
    let mut config = Config::default();
    config
        .permissions
        .rules
        .insert("cargo test".into(), PermissionAction::Deny);
    let args = json!({
        "command": "cargo test --lib denied",
        "cwd": workspace,
        "__ct_session_id": "deny-test",
    });
    let blocked =
        super::evaluate_tool_invocation_with_config(&config, "bash", &args).expect("denied");
    assert_eq!(blocked.metadata["policy_outcome"], "deny");
}