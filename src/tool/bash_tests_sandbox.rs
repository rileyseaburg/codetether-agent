use super::super::{BashTool, Tool};
use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn sandboxed_bash_timeout() {
    let tool = BashTool {
        timeout_secs: 1,
        sandboxed: true,
        default_cwd: None,
    };
    let result = tool
        .execute(json!({ "command": "sleep 30" }))
        .await
        .unwrap();
    assert!(!result.success);
}

#[tokio::test]
async fn unsandboxed_bash_requires_explicit_unsafe_authority() {
    let _lock = lock_env();
    let _env = ScopedEnv::access(AccessMode::Full);
    let tool = BashTool {
        timeout_secs: 10,
        sandboxed: false,
        default_cwd: None,
    };
    let result = tool
        .execute(json!({ "command": "echo unsafe path" }))
        .await
        .unwrap();
    assert!(!result.success);
    assert!(result.output.contains("explicit unsafe fallback"));
}