impl OpenAiCodexProvider {
    fn append_responses_assistant(
        message: &Message,
        input: &mut Vec<Value>,
        known_calls: &mut std::collections::HashSet<String>,
    ) {
        Self::append_responses_reasoning(message, input);
        let text = Self::message_text(message, "");
        if !text.is_empty() {
            input.push(json!({
                "type": "message",
                "role": "assistant",
                "content": [{ "type": "output_text", "text": text }],
            }));
        }
        for part in &message.content {
            if let ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } = part
            {
                known_calls.insert(id.clone());
                input.push(json!({
                    "type": "function_call",
                    "call_id": id,
                    "name": name,
                    "arguments": arguments,
                }));
            }
        }
    }
}
