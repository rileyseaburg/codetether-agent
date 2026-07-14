impl OpenAiCodexProvider {
    fn append_response_output_tool(item: &Value, parts: &mut Vec<ContentPart>) -> bool {
        let call_id = item
            .get("call_id")
            .or_else(|| item.get("id"))
            .and_then(Value::as_str);
        let name = item.get("name").and_then(Value::as_str);
        let arguments = item.get("arguments").and_then(Value::as_str);
        let (Some(call_id), Some(name), Some(arguments)) = (call_id, name, arguments) else {
            return false;
        };
        parts.push(ContentPart::ToolCall {
            id: call_id.to_string(),
            name: name.to_string(),
            arguments: arguments.to_string(),
            thought_signature: None,
        });
        true
    }
}
