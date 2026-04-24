use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn tetherscript_err_result_becomes_failed_tool_result() {
    let tool = TetherScriptPluginTool::new();
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
