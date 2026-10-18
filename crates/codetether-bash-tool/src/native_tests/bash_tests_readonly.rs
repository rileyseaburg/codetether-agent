use super::super::{BashTool, Tool};
use serde_json::json;

#[tokio::test]
async fn sandboxed_read_only_date_uses_safe_direct_execution() {
    let tool = BashTool::configured(10, true, None);
    let result = tool
        .execute(json!({ "command": "date +%Y" }))
        .await
        .unwrap();
    assert!(result.success);
    assert_eq!(result.metadata.get("sandboxed"), Some(&json!(false)));
    assert_eq!(result.metadata.get("unsafe_execution"), Some(&json!(false)));
    assert_eq!(
        result.metadata.get("unsafe_fallback_reason"),
        Some(&json!("read_only_command"))
    );
}
