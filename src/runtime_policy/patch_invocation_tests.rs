use super::evaluate_tool_invocation_with_config;
use crate::approval::{ApprovalStore, test_env::ENV_LOCK};
use crate::config::Config;
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
fn patch_write_receipt_satisfies_runtime_gate() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let request = store
        .create_request("apply_patch", "write", "src/lib.rs", "patch write")
        .expect("request");
    store.approve(&request.id, "riley", "ok").expect("approve");
    let args = json!({"patch": sample_patch(), "approval_id": request.id});
    assert!(
        evaluate_tool_invocation_with_config(&Config::default(), "apply_patch", &args).is_none()
    );
}

fn sample_patch() -> &'static str {
    "--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1,1 +1,1 @@\n-old\n+new\n"
}
