impl OpenAiCodexProvider {
    #[allow(dead_code)]
    fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        codetether_codex_protocol::CodexProtocol::convert_tools(tools)
    }
}
