//! Test `browser_render.tether` through the CodeTether tool runtime.

#[cfg(test)]
#[cfg(feature = "tetherscript")]
mod tests {
    use crate::tool::Tool;
    use crate::tool::tetherscript::TetherScriptPluginTool;
    use serde_json::json;

    #[tokio::test]
    async fn render_hook_produces_viewport_text() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(json!({
                "path": "examples/tetherscript/browser_render.tether",
                "hook": "render",
                "args": ["<h1>Hello</h1><p>World</p>"]
            }))
            .await
            .unwrap();
        assert!(result.success, "hook failed: {}", result.output);
        let val = result.metadata.get("value").unwrap();
        assert_eq!(val["ok"], true);
        assert!(val["text"].as_str().unwrap().contains("Hello"));
    }

    #[tokio::test]
    async fn render_with_css_includes_styled_elements() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(json!({
                "path": "examples/tetherscript/browser_render.tether",
                "hook": "render_with_css",
                "args": ["<h1>Title</h1>", "h1 { color: red }"]
            }))
            .await
            .unwrap();
        assert!(result.success, "hook failed: {}", result.output);
        assert!(result.output.contains("Title"));
    }
}
