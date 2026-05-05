//! Test `browser_dom.tether` through the CodeTether tool runtime.

#[cfg(test)]
#[cfg(feature = "tetherscript")]
mod tests {
    use crate::tool::Tool;
    use crate::tool::tetherscript::TetherScriptPluginTool;
    use serde_json::json;

    #[tokio::test]
    async fn extract_text_returns_rendered_content() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(json!({
                "path": "examples/tetherscript/browser_dom.tether",
                "hook": "extract_text",
                "args": ["<h1>Title</h1><p>Body text</p>"]
            }))
            .await
            .unwrap();
        assert!(result.success, "hook failed: {}", result.output);
    }

    #[tokio::test]
    async fn query_returns_matching_elements() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(json!({
                "path": "examples/tetherscript/browser_dom.tether",
                "hook": "query",
                "args": ["<div class='x'>hi</div><div class='y'>bye</div>", ".x"]
            }))
            .await
            .unwrap();
        assert!(result.success, "hook failed: {}", result.output);
    }
}
