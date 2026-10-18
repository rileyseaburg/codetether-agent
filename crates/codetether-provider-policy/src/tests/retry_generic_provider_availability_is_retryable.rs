//! Original provider policy regression case.
use crate::is_retryable_upstream_error;

#[test]
fn generic_provider_availability_is_retryable() {
    let err = anyhow::anyhow!("temporary provider availability issue; retry the request");

    assert!(is_retryable_upstream_error(&err));
    assert!(!err.to_string().to_ascii_lowercase().contains("websocket"));
}
