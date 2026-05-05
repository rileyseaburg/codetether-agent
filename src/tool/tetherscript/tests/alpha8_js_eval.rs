//! Test `js_eval.tether` through the CodeTether tool runtime.

#[cfg(test)]
#[cfg(feature = "tetherscript")]
mod tests {
    use crate::tool::Tool;
    use crate::tool::tetherscript::TetherScriptPluginTool;
    use serde_json::json;

    #[tokio::test]
    async fn js_eval_returns_arithmetic_result() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(json!({
                "path": "examples/tetherscript/js_eval.tether",
                "hook": "eval",
                "args": ["2 + 3 * 4"]
            }))
            .await
            .unwrap();
        assert!(result.success, "hook failed: {}", result.output);
        let val = result.metadata.get("value").unwrap();
        assert_eq!(val["ok"], true);
        assert!(val["result"].as_str().unwrap().contains("14"));
    }

    #[tokio::test]
    async fn js_eval_json_parses_object() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(json!({
                "path": "examples/tetherscript/js_eval.tether",
                "hook": "eval_json",
                "args": ["({ name: 'test', count: 42 })"]
            }))
            .await
            .unwrap();
        assert!(result.success, "hook failed: {}", result.output);
    }
}
