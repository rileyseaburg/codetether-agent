impl OpenAiCodexProvider {
    fn convert_responses_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "name": t.name,
                    "description": t.description,
                    "strict": false,
                    "parameters": t.parameters
                })
            })
            .collect()
    }
}
