use serde_json::json;

use crate::tool::Tool;
use crate::tool::kiln::KilnPluginTool;

#[tokio::test]
async fn kiln_err_result_becomes_failed_tool_result() {
    let tool = KilnPluginTool::new();
    let result = tool
        .execute(json!({
            "source": r#"fn validate() { return Err("feature rejected") }"#,
            "hook": "validate"
        }))
        .await
        .unwrap();

    assert!(!result.success);
    assert!(result.output.contains("Err(\"feature rejected\")"));
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({"err": "feature rejected"}))
    );
}
