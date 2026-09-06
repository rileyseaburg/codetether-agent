impl OpenAiCodexProvider {
    fn append_responses_tool(
        message: &Message,
        input: &mut Vec<Value>,
        known_calls: &std::collections::HashSet<String>,
    ) {
        for (index, part) in message.content.iter().enumerate() {
            let ContentPart::ToolResult {
                tool_call_id,
                content,
            } = part
            else {
                continue;
            };
            if known_calls.contains(tool_call_id) {
                let siblings = &message.content[index + 1..];
                let end = siblings.iter().position(|part| {
                    matches!(part, ContentPart::ToolResult { .. })
                }).unwrap_or(siblings.len());
                let output = Self::responses_tool_output(&siblings[..end], content);
                input.push(json!({
                    "type": "function_call_output",
                    "call_id": tool_call_id,
                    "output": output,
                }));
            } else {
                tracing::warn!(
                    tool_call_id = %tool_call_id,
                    "Skipping orphaned function_call_output while building Codex responses input"
                );
            }
        }
    }
}