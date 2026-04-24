use serde_json::json;

use crate::tool::Tool;
use crate::tool::kiln::KilnPluginTool;

#[tokio::test]
async fn invalid_params_return_tool_error() {
    let tool = KilnPluginTool::new();
    let result = tool.execute(json!({"hook": 42})).await.unwrap();

    assert!(!result.success);
    assert!(result.output.contains("Invalid kiln_plugin params"));
}
