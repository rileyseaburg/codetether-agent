//! Provider regression coverage.

use super::*;

#[test]
fn supports_openai_compatible_provider_without_api_key() {
    let provider = OpenAIProvider::with_base_url_optional_key(
        None,
        "http://localhost:8080/v1".to_string(),
        "huggingface",
    )
    .expect("provider should initialize without API key");

    assert_eq!(provider.name(), "huggingface");
}
