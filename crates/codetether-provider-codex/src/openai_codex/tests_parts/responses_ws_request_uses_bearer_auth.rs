#[test]
fn responses_ws_request_uses_bearer_auth() {
    let request = OpenAiCodexProvider::build_responses_ws_request_with_base_url(
        "wss://example.com/v1/responses",
        "test-token",
    )
    .expect("request should build");

    assert_eq!(request.uri().to_string(), "wss://example.com/v1/responses");
    assert_eq!(
        request
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok()),
        Some("Bearer test-token")
    );
    assert_eq!(
        request
            .headers()
            .get("User-Agent")
            .and_then(|v| v.to_str().ok()),
        Some("codetether-responses-ws/1.0")
    );
}
