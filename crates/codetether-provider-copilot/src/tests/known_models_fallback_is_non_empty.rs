//! Existing regression test.
use super::*;

#[test]
fn known_models_fallback_is_non_empty() {
    let provider = CopilotProvider::new("test-token".to_string(), "test-client".into()).unwrap();
    let models = provider.known_models();
    assert!(!models.is_empty());
    // All fallback models should support tools
    assert!(models.iter().all(|m| m.supports_tools));
}
