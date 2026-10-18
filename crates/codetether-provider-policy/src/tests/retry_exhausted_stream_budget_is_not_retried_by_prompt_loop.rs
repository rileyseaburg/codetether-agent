//! Original provider policy regression case.
use crate::is_retryable_upstream_error;

#[test]
fn exhausted_stream_budget_is_not_retried_by_prompt_loop() {
    let err =
        anyhow::anyhow!("stream retry limit exhausted after 5 retries: websocket connection reset");
    assert!(!is_retryable_upstream_error(&err));
}
