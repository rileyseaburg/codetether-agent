impl OpenAiCodexProvider {
    fn responses_tool_output(parts: &[ContentPart], text: &str) -> Value {
        let mut content = parts
            .iter()
            .filter_map(|part| match part {
                ContentPart::Image { url, .. } => Some(json!({
                    "type": "input_image",
                    "image_url": url,
                    "detail": "auto",
                })),
                _ => None,
            })
            .collect::<Vec<_>>();
        if content.is_empty() {
            return Value::String(text.to_string());
        }
        content.push(json!({"type": "input_text", "text": text}));
        Value::Array(content)
    }
}