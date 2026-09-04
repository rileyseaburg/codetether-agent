use super::evaluate_tool_invocation_with_config;
use crate::approval::{
    ApprovalStore,
    test_env::{ScopedEnv, lock_env},
};
use crate::config::{AccessMode, Config};
use serde_json::json;

#[test]
fn justification_edits_do_not_change_the_approval_resource() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let first = json!({"command": "cargo test", "justification": "first wording"});
    let blocked = evaluate_tool_invocation_with_config(&Config::default(), "bash", &first)
        .expect("approval required");
    let id = blocked.metadata["approval_request_id"].as_str().unwrap().to_string();
    store.approve(&id, "riley", "ok").expect("approve");
    let retry = json!({"command": "cargo test", "justification": "reworded", "approval_id": id});
    assert!(evaluate_tool_invocation_with_config(&Config::default(), "bash", &retry).is_none());
}

#[test]
fn approve_mode_does_not_demand_justification() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Approve);
    let config = Config {
        access_mode: Some(AccessMode::Approve),
        ..Config::default()
    };
    let args = json!({"command": "cargo test", "cwd": data.path().display().to_string()});
    let blocked = evaluate_tool_invocation_with_config(&config, "bash", &args);
    let code = blocked.map(|result| result.metadata["error_code"].clone());
    assert_ne!(code, Some(json!("TOOL_JUSTIFICATION_REQUIRED")));
}
