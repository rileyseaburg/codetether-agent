//! Indexer behavior regression.

use super::*;

#[test]
fn retryable_embedding_error_detection_matches_transient_signals() {
    assert!(is_retryable_embedding_error(&anyhow!(
        "HTTP 429 too many requests"
    )));
    assert!(is_retryable_embedding_error(&anyhow!("gateway timeout")));
    assert!(!is_retryable_embedding_error(&anyhow!(
        "invalid embedding model"
    )));
}
