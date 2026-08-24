use crate::approval::{ApprovalStore, session_grants, test_env::ScopedEnv, test_env::lock_env};
use crate::config::{AccessMode, Config};
use serde_json::json;

#[test]
fn reusable_session_grant_cannot_authorize_escalation() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    session_grants::reset();
    let args = json!({
        "cmd": "true",
        "workdir": data.path(),
        "sandbox_permissions": "require_escalated",
        "__ct_session_id": "escalation-session",
    });
    let blocked = crate::runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(),
        "exec_command",
        &args,
    )
    .expect("approval required");
    let id = blocked.metadata["approval_request_id"].as_str().expect("id");
    let receipt = ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "session")
        .expect("approve");
    session_grants::grant(&receipt);
    assert!(
        crate::runtime_policy::evaluate_tool_invocation_with_config(
            &Config::default(),
            "exec_command",
            &args,
        )
        .is_some()
    );
    session_grants::reset();
}