use super::{Registry, network_env_support as env, network_test_support as tcp};
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, exec_command::ExecCommandTool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn network_escalation_approval_cannot_override_session_denial() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = env::DisabledNetwork::set();
    let listener = tcp::listener();
    let mut args = json!({
        "cmd": tcp::command(&listener),
        "workdir": data.path(),
        "sandbox_permissions": "require_escalated",
        "yield_time_ms": 1_000,
        "__ct_parent_workspace": data.path(),
        "__ct_session_id": "network-escalation-test",
    });
    crate::tool::network_access::bind_trusted(&mut args, false);
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), None);

    let denied = tool
        .execute(args.clone())
        .await
        .expect("unapproved result");
    assert!(!denied.success);
    assert!(!tcp::received(&listener).await, "unapproved network reached host");
    let request_id = denied.metadata["approval_request_id"]
        .as_str()
        .expect("approval request id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(request_id, "test", "network escalation")
        .expect("approve");
    args["approval_id"] = json!(request_id);

    let approved = tool
        .execute(args.clone())
        .await
        .expect("approved result");
    assert!(!approved.success, "network denial was bypassed");
    assert!(!tcp::received(&listener).await, "approved network reached host");

    let replay = tool.execute(args).await.expect("replay result");
    assert!(!replay.success);
    assert!(replay.output.contains("approval"));
}