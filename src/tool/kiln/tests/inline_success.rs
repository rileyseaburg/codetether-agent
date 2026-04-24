use serde_json::json;

use crate::tool::Tool;
use crate::tool::kiln::KilnPluginTool;

#[tokio::test]
async fn executes_inline_kiln_hook_through_tool_trait() {
    let tool = KilnPluginTool::new();
    let result = tool
        .execute(json!({
            "source": r#"
fn validate(name) {
    println("kiln saw " + name)
    return Ok("hello " + name)
}
"#,
            "hook": "validate",
            "args": ["codetether"]
        }))
        .await
        .unwrap();

    assert!(result.success);
    assert!(result.output.contains("kiln saw codetether"));
    assert!(result.output.contains("Ok(hello codetether)"));
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({"ok": "hello codetether"}))
    );
}
