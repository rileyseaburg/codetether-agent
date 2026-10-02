//! Approval-to-backend handoff uses the real store and policy checks.

use crate::approval::{
    ApprovalStore,
    test_env::{ScopedEnv, lock_env},
};
use crate::config::{AccessMode, Config, SandboxMode};
use crate::runtime_policy::{approved_invocation, evaluate_tool_invocation_with_config};
use serde_json::json;

#[test]
fn approved_escalation_survives_progress_correlation() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let config = Config {
        sandbox_mode: Some(SandboxMode::WorkspaceWrite),
        ..Config::default()
    };
    let mut args = json!({"cmd": "cargo update", "workdir": data.path(),
        "sandbox_permissions": "require_escalated", "justification": "test approved handoff"});
    let blocked = evaluate_tool_invocation_with_config(&config, "exec_command", &args)
        .expect("approval required");
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    store
        .approve(id, "test-user", "approved exact invocation")
        .expect("approve");
    args["approval_id"] = json!(id);
    assert!(approved_invocation("exec_command", &args));
    args["_tool_call_id"] = json!("injected-by-execution-runner");
    assert!(approved_invocation("exec_command", &args));
    assert!(!super::super::unapproved_escalation(&args));
    assert!(!super::super::enabled(&config, "cargo update", &args));
    args["cmd"] = json!("cargo publish");
    assert!(super::super::unapproved_escalation(&args));
}

#[test]
fn forged_approval_id_cannot_authorize_escalation() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let args = json!({"cmd": "cargo update", "sandbox_permissions": "require_escalated",
        "approval_id": "unissued-id", "_tool_call_id": "progress-id"});
    assert!(super::super::unapproved_escalation(&args));
}
