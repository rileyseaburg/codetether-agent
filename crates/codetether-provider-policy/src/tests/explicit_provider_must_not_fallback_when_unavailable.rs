//! Original session provider-policy regression case.
use crate::resolve_provider_for_session_request;

#[test]
fn explicit_provider_must_not_fallback_when_unavailable() {
    let providers = vec!["zai", "openai"];
    let result = resolve_provider_for_session_request(&providers, Some("local_cuda"));
    assert!(result.is_err());
    assert!(
        result
            .err()
            .map(|e| e.to_string())
            .unwrap_or_default()
            .contains("selected explicitly but is unavailable")
    );
}
