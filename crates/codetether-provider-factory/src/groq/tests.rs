//! Groq identity, default endpoint and required credentials at the factory boundary.
use codetether_provider_core::model_identity::resolve;
use codetether_secrets::ProviderSecrets;

#[test]
fn groq_vault_dispatch_uses_default_endpoint_without_override() {
    let secrets = ProviderSecrets::default();
    let provider = crate::special::dispatch_openai_compat("fixture-key", &secrets, "groq")
        .expect("Groq must not require a base_url override");
    assert_eq!(provider.name(), "groq");
    assert_eq!(
        codetether_provider_openai::GROQ_BASE_URL,
        "https://api.groq.com/openai/v1"
    );
    assert_eq!(
        resolve("groq", "openai/gpt-oss-120b"),
        "openai/gpt-oss-120b"
    );
}

#[test]
fn groq_rejects_empty_credentials_and_supports_explicit_endpoint() {
    for key in ["", "   ", "\n"] {
        assert!(super::provider(key, None).is_err());
    }
    let secrets = ProviderSecrets {
        base_url: Some("http://127.0.0.1:1/v1".into()),
        ..Default::default()
    };
    let provider = crate::special::dispatch_openai_compat("fixture-key", &secrets, "groq").unwrap();
    assert_eq!(provider.name(), "groq");
}
