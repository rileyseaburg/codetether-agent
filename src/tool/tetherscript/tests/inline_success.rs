use serde_json::json;

use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn executes_inline_tetherscript_hook_through_tool_trait() {
    let tool = TetherScriptPluginTool::new();
    let result = super::support::execute(
        &tool,
        json!({
            "source": r#"
fn validate(name) {
    println("tetherscript saw " + name)
    return Ok("hello " + name)
}
"#,
            "hook": "validate",
            "args": ["codetether"]
        }),
    )
    .await;

    assert!(result.success, "{}", result.output);
    assert!(result.output.contains("tetherscript saw codetether"));
    assert!(result.output.contains("Ok(hello codetether)"));
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({"ok": "hello codetether"}))
    );
}
