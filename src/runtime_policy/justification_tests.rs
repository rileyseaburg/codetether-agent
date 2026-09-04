use super::evaluate_tool_invocation_with_config;
use crate::approval::{
    ApprovalStore,
    test_env::{ScopedEnv, lock_env},
};
use crate::config::{AccessMode, Config};
use serde_json::json;

#[test]
fn ask_mode_blocks_prompt_until_model_justifies_request() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let args = json!({"command": "cargo test", "cwd": data.path().display().to_string()});
    let blocked = evaluate_tool_invocation_with_config(&Config::default(), "bash", &args)
        .expect("blocked");
    assert_eq!(blocked.metadata["error_code"], "TOOL_JUSTIFICATION_REQUIRED");
    assert_eq!(blocked.metadata["justification_required"], true);
    assert!(!blocked.metadata.contains_key("approval_request_id"));
    assert!(blocked.output.contains("\"justification\""));
}

#[test]
fn ask_mode_justification_becomes_stored_approval_reason() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let args = json!({
        "command": "cargo test",
        "cwd": data.path().display().to_string(),
        "justification": "  run the focused suite the user asked for  "
    });
    let blocked = evaluate_tool_invocation_with_config(&Config::default(), "bash", &args)
        .expect("approval required");
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    assert_eq!(
        blocked.metadata["approval_justification"],
        "run the focused suite the user asked for"
    );
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let request = store.request(id).expect("lookup").expect("stored request");
    assert_eq!(request.reason, "run the focused suite the user asked for");
}
