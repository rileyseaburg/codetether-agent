impl OpenAiCodexProvider {
    fn append_response_output_text(item: &Value, parts: &mut Vec<ContentPart>) {
        let Some(content) = item.get("content").and_then(Value::as_array) else {
            return;
        };
        let text = content
            .iter()
            .filter(|segment| {
                matches!(
                    segment.get("type").and_then(Value::as_str),
                    Some("output_text" | "text")
                )
            })
            .filter_map(|segment| segment.get("text").and_then(Value::as_str))
            .collect::<String>();
        if !text.is_empty() {
            parts.push(ContentPart::Text { text });
        }
    }
}
