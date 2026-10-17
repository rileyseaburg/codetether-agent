#[test]
fn responses_ws_request_includes_chatgpt_account_id_when_provided() {
    let request = OpenAiCodexProvider::build_responses_ws_request_with_base_url_and_account_id(
        "wss://example.com/v1/responses",
        "test-token",
        Some("org_123"),
    )
    .expect("request should build");

    assert_eq!(
        request
            .headers()
            .get("chatgpt-account-id")
            .and_then(|v| v.to_str().ok()),
        Some("org_123")
    );
}
