//! Original provider policy regression case.
use crate::is_retryable_upstream_error;

#[test]
fn rate_limits_remain_retryable() {
    let err = anyhow::anyhow!("provider returned status code 429");

    assert!(is_retryable_upstream_error(&err));
}
