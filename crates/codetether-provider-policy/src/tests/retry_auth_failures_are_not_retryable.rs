//! Original provider policy regression case.
use crate::is_retryable_upstream_error;

#[test]
fn auth_failures_are_not_retryable() {
    let err = anyhow::anyhow!("Bedrock stream error (403 Forbidden): Authentication failed");

    assert!(!is_retryable_upstream_error(&err));
}
