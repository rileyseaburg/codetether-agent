use crate::images::authenticate::retry_cached_anonymous;
use anyhow::anyhow;

#[test]
fn recovery_only_retries_anonymous_bootstrap_from_cache() {
    let anonymous = anyhow!("image_session_not_authenticated");
    assert!(retry_cached_anonymous(true, &anonymous));
    assert!(!retry_cached_anonymous(false, &anonymous));
    for failure in [
        "image_session_http_error",
        "image_session_metadata_missing",
        "image_generation_protocol_error",
        "image_transport_failed",
    ] {
        assert!(!retry_cached_anonymous(true, &anyhow!(failure)));
        assert!(!retry_cached_anonymous(false, &anyhow!(failure)));
    }
}
