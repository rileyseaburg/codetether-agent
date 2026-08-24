use crate::approval::{ApprovalStore, session_grants, test_env::lock_env};
use crate::config::Config;
use serde_json::json;

struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        session_grants::reset();
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[test]
fn patch_session_grant_does_not_cross_session_ids() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    session_grants::reset();
    let _guard = Guard;
    let patch = "--- a/file.txt\n+++ b/file.txt\n@@ -1 +1 @@\n-old\n+new\n";
    let args_a = json!({"patch": patch, "__ct_session_id": "session-a"});
    let blocked =
        super::evaluate_tool_invocation_with_config(&Config::default(), "apply_patch", &args_a)
            .expect("approval required");
    let id = blocked.metadata["approval_request_id"].as_str().unwrap();
    let receipt = ApprovalStore::open_default()
        .unwrap()
        .approve(id, "test", "session approval")
        .unwrap();
    session_grants::grant(&receipt);

    assert!(
        super::evaluate_tool_invocation_with_config(&Config::default(), "apply_patch", &args_a)
            .is_none()
    );
    let args_b = json!({"patch": patch, "__ct_session_id": "session-b"});
    assert!(
        super::evaluate_tool_invocation_with_config(&Config::default(), "apply_patch", &args_b)
            .is_some()
    );
}
