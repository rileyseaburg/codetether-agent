use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn executes_inline_tetherscript_hook_through_tool_trait() {
    let tool = TetherScriptPluginTool::new();
    let result = tool
        .execute(json!({
            "source": r#"
fn validate(name) {
    println("tetherscript saw " + name)
    return Ok("hello " + name)
}
"#,
            "hook": "validate",
            "args": ["codetether"]
        }))
        .await
        .unwrap();

    assert!(result.success);
    assert!(result.output.contains("tetherscript saw codetether"));
    assert!(result.output.contains("Ok(hello codetether)"));
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({"ok": "hello codetether"}))
    );
}
