//! Direct Bash refuses receipts whose network authority cannot be enforced.

use crate::approval::{ApprovalStore, test_env::{ScopedEnv, lock_env}};
use crate::config::AccessMode;
use serde_json::json;

struct UnsafeEnv;
impl UnsafeEnv {
    fn set() -> Self {
        unsafe { std::env::set_var("CODETETHER_ALLOW_UNSAFE_SANDBOX_FALLBACK", "1") };
        Self
    }
}
impl Drop for UnsafeEnv {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_ALLOW_UNSAFE_SANDBOX_FALLBACK") };
    }
}

#[test]
fn network_false_receipt_is_not_consumed_by_direct_fallback() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _unsafe = UnsafeEnv::set();
    let mut args = json!({
        "command": "printf blocked",
        "__ct_session_id": "session-a",
        "__ct_parent_workspace": data.path(),
    });
    crate::tool::network_access::bind_trusted(&mut args, false);
    let scope = crate::runtime_policy::invocation_scope::for_tool("bash", &args);
    let store = ApprovalStore::open_default().expect("store");
    let request = store.create_request("bash", scope.action, &scope.resource, "test").unwrap();
    store.approve(&request.id, "test", "once").unwrap();
    args["approval_id"] = json!(request.id);

    let error = super::super::bash_sandbox_policy::authorize(false, "printf blocked", &args)
        .expect_err("direct fallback must reject network-false authority");
    assert!(error.to_string().contains("network"));
    assert!(store.claim(
        &request.id, "bash", scope.action, &scope.resource, "test claimant",
    ).is_ok());
}