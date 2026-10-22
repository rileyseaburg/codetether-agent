use super::OpenRouterProvider;

#[test]
fn parses_embedded_error_body() {
    let body = r#"{"error":{"message":"Internal Server Error","code":500}}"#;
    let message = OpenRouterProvider::parse_error_body(body);

    assert_eq!(
        message.as_deref(),
        Some("OpenRouter API error: Internal Server Error (code: 500)")
    );
}

#[test]
fn ignores_success_body_without_error_envelope() {
    let body = r#"{
            "id":"chatcmpl-123",
            "choices":[{
                "message":{"role":"assistant","content":"ok"},
                "finish_reason":"stop"
            }]
        }"#;

    assert_eq!(OpenRouterProvider::parse_error_body(body), None);
}
