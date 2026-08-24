use serde_json::json;

use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn renders_trace_title_with_tera() {
    let tool = TetherScriptPluginTool::new();
    let result = super::support::execute(
        &tool,
        json!({
            "path": "examples/tetherscript/tera_probe.tether",
            "hook": "render_title",
            "args": ["Trace Lens"]
        }),
    )
    .await;

    assert!(result.success, "hook failed: {}", result.output);
    assert_eq!(result.metadata["value"], "<title>Trace Lens</title>");
}
