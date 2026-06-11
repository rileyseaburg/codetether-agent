use crate::approval::{ApprovalStore, session_command_grants, test_env::ENV_LOCK};
use crate::config::{Config, PermissionAction};
use crate::runtime_policy::evaluate_tool_invocation_with_config;
use serde_json::json;

struct EnvGuard;

impl Drop for EnvGuard {
    fn drop(&mut self) {
        session_command_grants::reset();
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

fn setup() -> (tempfile::TempDir, EnvGuard) {
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    session_command_grants::reset();
    (data, EnvGuard)
}

#[test]
fn proposed_prefix_session_approval_allows_future_matching_command() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let (data, _env) = setup();
    let store = ApprovalStore::open_default().expect("store");
    let args = json!({
        "command": "cargo test --lib first",
        "prefix_rule": ["cargo", "test"],
        "cwd": data.path().display().to_string()
    });
    let blocked = evaluate_tool_invocation_with_config(&Config::default(), "bash", &args)
        .expect("approval required");
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("id");
    store.approve(id, "riley", "ok").expect("approve");
    session_command_grants::grant_for_request(id);
    let next = json!({"command": "cargo test --lib second"});
    assert!(evaluate_tool_invocation_with_config(&Config::default(), "bash", &next).is_none());
}

#[test]
fn configured_deny_overrides_session_prefix() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let (_data, _env) = setup();
    session_command_grants::remember_request("approval-1", vec!["cargo test".into()]);
    session_command_grants::grant_for_request("approval-1");
    let mut config = Config::default();
    let rules = &mut config.permissions.rules;
    rules.insert("cargo test".into(), PermissionAction::Deny);
    let args = json!({"command": "cargo test --lib denied"});
    let blocked = evaluate_tool_invocation_with_config(&config, "bash", &args).expect("denied");
    assert_eq!(blocked.metadata["policy_outcome"], "deny");
}
