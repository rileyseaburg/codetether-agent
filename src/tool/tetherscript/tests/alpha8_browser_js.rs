//! Test `browser_js.tether` through the CodeTether tool runtime.

#[cfg(test)]
#[cfg(feature = "tetherscript")]
mod tests {
    use crate::tool::Tool;
    use crate::tool::tetherscript::TetherScriptPluginTool;
    use serde_json::json;

    #[tokio::test]
    #[ignore = "alpha.8 browser_eval_js builtin not yet functional"]
    async fn eval_js_mutates_dom() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(json!({
                "path": "examples/tetherscript/browser_js.tether",
                "hook": "eval_js",
                "args": [
                    "<div id=\"app\"></div>",
                    "document.getElementById(\"app\").textContent = \"hello\""
                ]
            }))
            .await
            .unwrap();
        assert!(result.success, "hook failed: {}", result.output);
        let val = result.metadata.get("value").unwrap();
        assert_eq!(val["value"], "hello");
    }

    #[tokio::test]
    #[ignore = "alpha.8 browser_compatibility_report builtin not yet functional"]
    async fn compat_returns_feature_list() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(json!({
                "path": "examples/tetherscript/browser_js.tether",
                "hook": "compat",
                "args": []
            }))
            .await
            .unwrap();
        assert!(result.success, "hook failed: {}", result.output);
        let val = result.metadata.get("value").unwrap();
        let features = val.as_array().unwrap();
        assert!(features.iter().any(|f| f == "document"));
    }
}
