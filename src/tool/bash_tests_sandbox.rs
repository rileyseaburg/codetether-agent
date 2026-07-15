use super::super::{BashTool, Tool};
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
async fn unsandboxed_bash_reports_unsafe_metadata() {
    let tool = BashTool {
        timeout_secs: 10,
        sandboxed: false,
        default_cwd: None,
    };
    let result = tool
        .execute(json!({ "command": "echo unsafe path" }))
        .await
        .unwrap();
    assert!(result.success);
    assert_eq!(result.metadata.get("sandboxed"), Some(&json!(false)));
    assert_eq!(result.metadata.get("unsafe_execution"), Some(&json!(true)));
}
