use crate::approval::{ApprovalStore, test_env::lock_env};
use crate::config::Config;
use crate::runtime_policy::{approved_invocation, evaluate_tool_invocation_with_config};
use serde_json::{Value, json};

#[test]
fn progress_metadata_preserves_approved_invocation_scope() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = DataDirGuard::set(data.path());
    let store = ApprovalStore::open_default().expect("store");
    let mut args = json!({
        "cmd": "curl https://example.com",
        "__ct_lease_owner": "turn-1",
        "sandbox_permissions": "require_escalated"
    });
    let blocked = evaluate_tool_invocation_with_config(&Config::default(), "exec_command", &args)
        .expect("approval required");
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    store
        .approve(id, "test", "network access")
        .expect("approve");
    args["approval_id"] = json!(id);
    args["_tool_call_id"] = json!("call-1");
    args["__ct_lease_owner"] = json!("turn-2");

    assert!(approved_invocation("exec_command", &args));
    args["cmd"] = Value::String("curl https://other.example.com".into());
    assert!(!approved_invocation("exec_command", &args));
}

struct DataDirGuard;

impl DataDirGuard {
    fn set(path: &std::path::Path) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        Self
    }
}

impl Drop for DataDirGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}
