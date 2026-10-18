//! Original session provider-policy regression case.
use crate::choose_default_provider;

#[test]
fn default_provider_prefers_openai() {
    let providers = vec!["zai", "openai"];
    assert_eq!(choose_default_provider(&providers), Some("openai"));
}
