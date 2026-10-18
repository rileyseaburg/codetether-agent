//! Original provider policy regression case.
use crate::is_retryable_upstream_error;

#[test]
fn codex_server_overload_is_retryable() {
    let err = anyhow::anyhow!(
        "stream faulted (transient=false): Our servers are currently overloaded. Please try again later."
    );
    assert!(is_retryable_upstream_error(&err));
}
