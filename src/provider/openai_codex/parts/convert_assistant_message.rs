impl OpenAiCodexProvider {
    fn convert_assistant_message(message: &Message) -> Value {
        let text = Self::message_text(message, "");
        let tool_calls: Vec<Value> = message
            .content
            .iter()
            .filter_map(|part| match part {
                ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                    ..
                } => Some(json!({
                    "id": id,
                    "type": "function",
                    "function": { "name": name, "arguments": arguments },
                })),
                _ => None,
            })
            .collect();
        if tool_calls.is_empty() {
            return json!({ "role": "assistant", "content": text });
        }
        let content = if text.is_empty() {
            Value::Null
        } else {
            json!(text)
        };
        json!({ "role": "assistant", "content": content, "tool_calls": tool_calls })
    }
}
