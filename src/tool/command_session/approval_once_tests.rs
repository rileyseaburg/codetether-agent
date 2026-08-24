use super::Registry;
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::Tool;
use crate::tool::exec_command::ExecCommandTool;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn approved_exec_command_claims_before_start_and_rejects_replay() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({
        "cmd": "true",
        "workdir": data.path().display().to_string(),
        "sandbox_permissions": "require_escalated",
        "__ct_parent_workspace": data.path(),
        "__ct_session_id": "approval-once-test",
    });
    crate::tool::network_access::bind_trusted(&mut args, false);
    let scope = crate::runtime_policy::invocation_scope::for_tool("exec_command", &args);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request(
            "exec_command",
            scope.action,
            &scope.resource,
            "test escalation",
        )
        .expect("request");
    store.approve(&request.id, "test", "ok").expect("approve");
    args["approval_id"] = json!(request.id);
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), None);

    let first = tool.execute(args.clone()).await.expect("first execution");
    assert!(first.success, "{}", first.output);

    let replay = tool.execute(args).await.expect("replay result");
    assert!(!replay.success);
}