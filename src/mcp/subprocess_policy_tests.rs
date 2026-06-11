use super::{guard, policy_args};
use crate::approval::{ApprovalStore, test_env::ENV_LOCK};

#[path = "subprocess_policy_alias_tests.rs"]
mod alias;

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

#[tokio::test]
async fn subprocess_spawn_requires_policy_approval() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    assert!(guard("npx", &["server"], None).await.is_err());
}

#[tokio::test]
async fn approved_mcp_receipt_allows_subprocess_spawn_gate() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let args = policy_args("npx", &["server"], None);
    let blocked = crate::runtime_policy::evaluate_tool_invocation("mcp", &args)
        .await
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id")
        .to_string();
    store.approve(&request_id, "riley", "ok").expect("approve");
    assert!(guard("npx", &["server"], Some(&request_id)).await.is_ok());
}
