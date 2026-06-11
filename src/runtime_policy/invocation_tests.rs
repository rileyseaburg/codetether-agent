use super::evaluate_tool_invocation_with_config;
use crate::approval::{ApprovalStore, test_env::ENV_LOCK};
use crate::config::{Config, PermissionProfile, PermissionProfileConfig};
use serde_json::json;

struct EnvGuard;

impl EnvGuard {
    fn data_dir(path: &std::path::Path) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[test]
fn approved_invocation_id_allows_required_tool() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let mut args = json!({"command": "cargo test", "cwd": data.path().display().to_string()});
    let blocked = evaluate_tool_invocation_with_config(&Config::default(), "bash", &args)
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id")
        .to_string();
    store.approve(&request_id, "riley", "ok").expect("approve");
    args["approval_id"] = json!(request_id);
    assert!(evaluate_tool_invocation_with_config(&Config::default(), "bash", &args).is_none());
}

#[test]
fn approved_invocation_id_does_not_override_disabled_profile() {
    let mut config = Config::default();
    config.permission_profile = Some(PermissionProfileConfig::Named(PermissionProfile::Disabled));
    let args = json!({"approval_id": "anything"});
    assert!(evaluate_tool_invocation_with_config(&config, "bash", &args).is_some());
}
