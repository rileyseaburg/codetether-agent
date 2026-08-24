use super::network_env_support::DisabledNetwork;
use crate::approval::{ApprovalStore, session_command_grants};
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::{AccessMode, Config};
use serde_json::json;

struct GrantGuard;

impl Drop for GrantGuard {
    fn drop(&mut self) {
        session_command_grants::reset();
    }
}

#[test]
fn prefix_grant_cannot_cross_reviewed_network_state() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = DisabledNetwork::set();
    session_command_grants::reset();
    let _grants = GrantGuard;
    let args = json!({
        "command": "cargo test --lib reviewed",
        "cwd": data.path(),
        "prefix_rule": ["cargo", "test"],
        "__ct_session_id": "network-prefix",
    });
    let blocked = crate::runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(), "bash", &args,
    )
    .expect("approval required");
    let id = blocked.metadata["approval_request_id"].as_str().expect("id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "network disabled")
        .expect("approve");
    session_command_grants::grant_for_request(id);

    DisabledNetwork::allow(true);
    assert!(crate::runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(), "bash", &args,
    ).is_some());
    DisabledNetwork::allow(false);
    assert!(crate::runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(), "bash", &args,
    ).is_none());
}