/// Builds a minimal request for reasoning-shape assertions.
fn reasoning_probe_request(model: &str) -> CompletionRequest {
    CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "hi".to_string(),
            }],
        }],
        tools: Vec::new(),
        model: model.to_string(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: Vec::new(),
    }
}

/// Asserts a request body carries Codex's `effort` + `summary` reasoning shape.
fn assert_reasoning_shape(container: &Value, context: &str) {
    let reasoning = container.get("reasoning").expect("reasoning object");
    assert_eq!(reasoning.get("effort").and_then(Value::as_str), Some("high"));
    assert_eq!(
        reasoning.get("summary").and_then(Value::as_str),
        Some("auto"),
        "{context}"
    );
}
