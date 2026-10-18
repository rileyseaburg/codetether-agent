//! Original provider policy regression case.
use crate::is_retryable_upstream_error;

#[test]
fn service_unavailable_is_retryable() {
    let err = anyhow::anyhow!("Bedrock stream error (503 Service Unavailable)");

    assert!(is_retryable_upstream_error(&err));
}
