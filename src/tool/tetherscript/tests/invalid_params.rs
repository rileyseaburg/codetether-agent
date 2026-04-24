use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn invalid_params_return_tool_error() {
    let tool = TetherScriptPluginTool::new();
    let result = tool.execute(json!({"hook": 42})).await.unwrap();

    assert!(!result.success);
    assert!(result.output.contains("Invalid tetherscript_plugin params"));
}
