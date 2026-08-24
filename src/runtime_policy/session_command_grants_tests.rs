use crate::approval::{ApprovalStore, session_command_grants, test_env::lock_env};
use crate::config::Config;
use crate::runtime_policy::evaluate_tool_invocation_with_config;
use serde_json::json;

const SESSION: &str = "session-command-test";
struct EnvGuard;

impl Drop for EnvGuard {
    fn drop(&mut self) {
        session_command_grants::reset();
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

fn setup() -> (tempfile::TempDir, EnvGuard) {
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    session_command_grants::reset();
    (data, EnvGuard)
}

#[test]
fn proposed_prefix_session_approval_allows_future_matching_command() {
    let _lock = lock_env();
    let (data, _env) = setup();
    let store = ApprovalStore::open_default().expect("store");
    let args = json!({
        "command": "cargo test --lib first",
        "__ct_session_id": SESSION,
        "prefix_rule": ["cargo", "test"],
        "cwd": data.path().display().to_string()
    });
    let blocked = evaluate_tool_invocation_with_config(&Config::default(), "bash", &args)
        .expect("approval required");
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("id");
    store.approve(id, "riley", "ok").expect("approve");
    session_command_grants::grant_for_request(id);
    let next = json!({
        "command": "cargo test --lib second",
        "__ct_session_id": SESSION,
        "cwd": data.path().display().to_string(),
    });
    assert!(evaluate_tool_invocation_with_config(&Config::default(), "bash", &next).is_none());
}
