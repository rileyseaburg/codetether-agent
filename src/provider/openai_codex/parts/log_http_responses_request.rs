impl OpenAiCodexProvider {
    fn log_http_responses_request(backend: &str, body: &Value) {
        let instructions_len = body
            .get("instructions")
            .and_then(Value::as_str)
            .map(str::len)
            .unwrap_or(0);
        let input_items = body
            .get("input")
            .and_then(Value::as_array)
            .map(Vec::len)
            .unwrap_or(0);
        let has_tools = body
            .get("tools")
            .and_then(Value::as_array)
            .is_some_and(|tools| !tools.is_empty());
        tracing::info!(
            backend,
            instructions_len,
            input_items,
            has_tools,
            "Sending HTTP responses request"
        );
    }
}
