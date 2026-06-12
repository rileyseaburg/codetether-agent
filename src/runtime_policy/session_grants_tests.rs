use super::evaluate_tool_invocation_with_config;
use crate::approval::{ApprovalStore, session_grants, test_env::lock_env};
use crate::config::Config;
use serde_json::json;

struct EnvGuard;

impl Drop for EnvGuard {
    fn drop(&mut self) {
        session_grants::reset();
        crate::approval::session_command_grants::reset();
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[test]
fn session_grant_allows_future_matching_invocation() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    let _env = EnvGuard;
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let args = json!({
        "command": "cargo test --lib session_grants",
        "cwd": data.path().display().to_string()
    });
    let blocked = evaluate_tool_invocation_with_config(&Config::default(), "bash", &args)
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");

    let receipt = store.approve(request_id, "riley", "ok").expect("approve");
    session_grants::grant(&receipt);

    assert!(evaluate_tool_invocation_with_config(&Config::default(), "bash", &args).is_none());
}

#[path = "session_command_grants_tests.rs"]
mod command_grants_tests;
