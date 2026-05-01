#[cfg(test)]
#[cfg(feature = "tetherscript")]
mod tests {
    use crate::tool::Tool;
    use crate::tool::tetherscript::TetherScriptPluginTool;

    #[tokio::test]
    async fn repair_null_reasoning_content_via_tool() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(serde_json::json!({
                "path": "examples/tetherscript/deepseek_repair.tether",
                "hook": "repair_msg",
                "args": [{
                    "role": "assistant",
                    "content": null,
                    "reasoning_content": null,
                    "tool_calls": [{"id": "call_1", "type": "function", "function": {"name": "grep", "arguments": "{}"}}]
                }]
            }))
            .await
            .unwrap();

        assert!(result.success, "hook failed: {}", result.output);
        let val = result.metadata.get("value").unwrap();
        assert_eq!(val["reasoning_content"], "");
    }

    #[tokio::test]
    async fn preserves_existing_reasoning_via_tool() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(serde_json::json!({
                "path": "examples/tetherscript/deepseek_repair.tether",
                "hook": "repair_msg",
                "args": [{
                    "role": "assistant",
                    "content": null,
                    "reasoning_content": "keep me",
                    "tool_calls": [{"id": "call_2"}]
                }]
            }))
            .await
            .unwrap();

        assert!(result.success);
        let val = result.metadata.get("value").unwrap();
        assert_eq!(val["reasoning_content"], "keep me");
    }

    #[tokio::test]
    async fn skips_non_assistant_via_tool() {
        let tool = TetherScriptPluginTool::new();
        let result = tool
            .execute(serde_json::json!({
                "path": "examples/tetherscript/deepseek_repair.tether",
                "hook": "repair_msg",
                "args": [{"role": "user", "content": "hello"}]
            }))
            .await
            .unwrap();

        assert!(result.success);
        let val = result.metadata.get("value").unwrap();
        assert!(val.get("reasoning_content").is_none());
    }
}
