//! Issue and decide exact escalation requests through the real approval store.

use crate::approval::{ApprovalStore, test_env::ScopedEnv};
use crate::config::{AccessMode, Config, SandboxMode};
use crate::runtime_policy::evaluate_tool_invocation_with_config;
use serde_json::{Value, json};

pub(super) fn command(cmd: &str, approve: bool) -> (tempfile::TempDir, ScopedEnv, Value) {
    let dir = tempfile::tempdir().unwrap();
    let env = ScopedEnv::data_dir_with_access(dir.path(), AccessMode::Ask);
    let config = Config {
        sandbox_mode: Some(SandboxMode::WorkspaceWrite),
        ..Config::default()
    };
    let mut args = json!({"cmd": cmd, "workdir": dir.path(), "login": false,
        "sandbox_permissions": "require_escalated", "yield_time_ms": 5000,
        "justification": "regression: exact approved filesystem writes"});
    let blocked = evaluate_tool_invocation_with_config(&config, "exec_command", &args).unwrap();
    let id = blocked.metadata["approval_request_id"].as_str().unwrap();
    let store = ApprovalStore::open(dir.path().join("approvals")).unwrap();
    if approve {
        store
            .approve(id, "test-user", "authorize exact invocation")
            .unwrap();
    } else {
        store
            .deny(id, "test-user", "deny exact invocation")
            .unwrap();
    }
    args["approval_id"] = json!(id);
    args["_tool_call_id"] = json!("progress-id-injected-after-decision");
    (dir, env, args)
}
