use super::super::{BashTool, Tool};
use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn sandboxed_read_only_date_retains_os_sandbox() {
    let _lock = lock_env();
    let _env = ScopedEnv::access(AccessMode::Full);
    let tool = BashTool {
        timeout_secs: 10,
        sandboxed: true,
        default_cwd: None,
    };
    let result = tool
        .execute(json!({ "command": "date +%Y" }))
        .await
        .unwrap();
    assert!(result.success);
    assert_eq!(result.metadata.get("sandboxed"), Some(&json!(true)));
}