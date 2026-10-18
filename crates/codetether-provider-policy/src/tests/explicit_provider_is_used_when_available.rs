//! Original session provider-policy regression case.
use crate::resolve_provider_for_session_request;

#[test]
fn explicit_provider_is_used_when_available() {
    let providers = vec!["local_cuda", "zai"];
    let result = resolve_provider_for_session_request(&providers, Some("local_cuda"));
    assert_eq!(result.ok(), Some("local_cuda"));
}
