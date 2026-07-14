impl OpenAiCodexProvider {
    fn convert_tool_message(message: &Message) -> Value {
        let Some(ContentPart::ToolResult {
            tool_call_id,
            content,
        }) = message.content.first()
        else {
            return json!({ "role": "tool", "content": "" });
        };
        json!({
            "role": "tool",
            "tool_call_id": tool_call_id,
            "content": content,
        })
    }
}
