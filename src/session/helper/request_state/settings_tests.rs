use super::model_supports_tools;
use crate::provider::ToolDefinition;

#[test]
fn gemini_web_advertises_executable_tools() {
    let tools = vec![ToolDefinition {
        name: "bash".into(),
        description: "Run a command".into(),
        parameters: serde_json::json!({"type": "object"}),
    }];
    let advertised =
        super::super::tools::advertised_tools(model_supports_tools("gemini-web"), &tools);
    assert_eq!(advertised[0].name, "bash");
}

#[test]
fn local_cuda_keeps_text_tool_bootstrap() {
    for provider in ["local-cuda", "local_cuda", "localcuda"] {
        assert!(!model_supports_tools(provider));
    }
}
