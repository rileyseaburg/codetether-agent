use super::network_env_support::DisabledNetwork;
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::{AccessMode, Config};
use serde_json::json;

#[tokio::test]
async fn approved_command_is_bound_to_reviewed_network_state() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = DisabledNetwork::set();
    let mut args = json!({
        "cmd": "touch network-scope",
        "workdir": data.path(),
    });
    let blocked = crate::runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(),
        "exec_command",
        &args,
    )
    .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(request_id, "test", "network disabled")
        .expect("approve");
    args["approval_id"] = json!(request_id);

    DisabledNetwork::allow(true);
    assert!(
        crate::runtime_policy::evaluate_tool_invocation_with_config(
            &Config::default(), "exec_command", &args,
        )
        .is_some()
    );
    DisabledNetwork::allow(false);
    assert!(crate::runtime_policy::approved_invocation("exec_command", &args));
}