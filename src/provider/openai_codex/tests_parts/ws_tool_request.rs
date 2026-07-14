fn ws_tool_request() -> CompletionRequest {
    CompletionRequest {
        messages: vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: "System prompt".to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "Inspect the repo".to_string(),
                }],
            },
        ],
        tools: vec![ToolDefinition {
            name: "read".to_string(),
            description: "Read a file".to_string(),
            parameters: json!({
                "type": "object",
                "properties": { "path": { "type": "string" } },
                "required": ["path"]
            }),
        }],
        model: "gpt-5.4".to_string(),
        temperature: None,
        top_p: None,
        max_tokens: Some(8192),
        stop: Vec::new(),
    }
}
