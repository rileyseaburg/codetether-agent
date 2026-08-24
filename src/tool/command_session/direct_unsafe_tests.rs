//! Explicit direct mode still requires exact approval before spawning.

use super::Registry;
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, exec_command::ExecCommandTool};
use serde_json::json;
use std::sync::Arc;

struct UnsafeEnv;
impl UnsafeEnv {
    fn set() -> Self {
        unsafe {
            std::env::set_var("CODETETHER_UNSANDBOXED_BASH", "1");
            std::env::set_var("CODETETHER_ALLOW_UNSAFE_SANDBOX_FALLBACK", "1");
            std::env::set_var("CODETETHER_ALLOW_NETWORK", "1");
        }
        Self
    }
}
impl Drop for UnsafeEnv {
    fn drop(&mut self) {
        for key in ["CODETETHER_UNSANDBOXED_BASH", "CODETETHER_ALLOW_UNSAFE_SANDBOX_FALLBACK", "CODETETHER_ALLOW_NETWORK"] {
            unsafe { std::env::remove_var(key) };
        }
    }
}

#[tokio::test]
async fn full_access_direct_mode_without_receipt_spawns_nothing() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Full);
    let _unsafe = UnsafeEnv::set();
    let cwd = std::env::current_dir().expect("cwd");
    let workspace = tempfile::tempdir_in(cwd).expect("workspace tempdir");
    let marker = workspace.path().join("must-not-exist");
    let result = ExecCommandTool::new(Arc::new(Registry::default()), None)
        .execute(json!({"cmd": "touch must-not-exist", "workdir": workspace.path()}))
        .await
        .expect("result");
    assert!(!result.success);
    assert!(!marker.exists());
}