use super::super::{BashTool, Tool};
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn sandboxed_bash_basic() {
    let _lock = lock_env();
    let _env = ScopedEnv::access(AccessMode::Approve);
    let cwd = tempfile::tempdir().expect("tempdir");
    let tool = BashTool {
        timeout_secs: 10,
        sandboxed: true,
        default_cwd: Some(cwd.path().to_path_buf()),
    };
    let result = tool
        .execute(json!({ "command": "mkdir x; echo sandbox; rmdir x" }))
        .await
        .unwrap();
    let ok = result.output.contains("sandbox");
    let denied = result.output.contains("OS sandbox unavailable");
    assert!(ok || denied);
    assert_eq!(result.metadata.get("sandboxed"), Some(&json!(true)));
}
