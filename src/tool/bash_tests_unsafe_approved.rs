//! Mandatory integration test for the deliberately unsafe direct fallback.

use super::super::{BashTool, Tool};
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use serde_json::json;

struct UnsafeFallback;

impl UnsafeFallback {
    fn enable() -> Self {
        unsafe { std::env::set_var("CODETETHER_ALLOW_UNSAFE_SANDBOX_FALLBACK", "1") };
        Self
    }
}

impl Drop for UnsafeFallback {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_ALLOW_UNSAFE_SANDBOX_FALLBACK") };
    }
}

#[tokio::test]
#[ignore = "mandatory explicit unsafe-fallback integration lane"]
async fn exact_approval_and_unsafe_setting_allow_one_direct_bash() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _unsafe = UnsafeFallback::enable();
    let tool = BashTool {
        timeout_secs: 10,
        sandboxed: false,
        default_cwd: None,
    };
    let mut args = json!({
        "command":"printf direct-approved", "__ct_session_id":"direct-bash-test"
    });
    crate::tool::network_access::bind_trusted(&mut args, true);
    let blocked = tool.execute(args.clone()).await.expect("approval result");
    let id = blocked.metadata["approval_request_id"].as_str().expect("id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "direct fallback")
        .expect("approve");
    args["approval_id"] = json!(id);
    let result = tool.execute(args.clone()).await.expect("direct result");
    assert!(result.success, "{}", result.output);
    assert_eq!(result.output, "direct-approved");
    assert!(!tool.execute(args).await.expect("replay").success);
}