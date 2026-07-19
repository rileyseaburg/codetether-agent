use super::retry_error::is_retryable_upstream_error;

#[test]
fn auth_failures_are_not_retryable() {
    let err = anyhow::anyhow!("Bedrock stream error (403 Forbidden): Authentication failed");

    assert!(!is_retryable_upstream_error(&err));
}

#[test]
fn rate_limits_remain_retryable() {
    let err = anyhow::anyhow!("provider returned status code 429");

    assert!(is_retryable_upstream_error(&err));
}

#[test]
fn service_unavailable_is_retryable() {
    let err = anyhow::anyhow!("Bedrock stream error (503 Service Unavailable)");

    assert!(is_retryable_upstream_error(&err));
}

#[test]
fn generic_provider_availability_is_retryable() {
    let err = anyhow::anyhow!("temporary provider availability issue; retry the request");

    assert!(is_retryable_upstream_error(&err));
    assert!(!err.to_string().to_ascii_lowercase().contains("websocket"));
}

#[test]
fn exhausted_stream_budget_is_not_retried_by_prompt_loop() {
    let err = anyhow::anyhow!(
        "stream retry limit exhausted after 5 retries: websocket connection reset"
    );
    assert!(!is_retryable_upstream_error(&err));
}
