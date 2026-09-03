//! Tests for Bedrock-hosted OpenAI model aliases.

use super::resolve_openai_alias;
use crate::provider::bedrock::BedrockProvider;

#[test]
fn short_aliases_map_to_openai_prefix() {
    assert_eq!(
        resolve_openai_alias("gpt-6-astra"),
        Some("openai.gpt-6-astra")
    );
    assert_eq!(
        resolve_openai_alias("gpt-5.6-sol"),
        Some("openai.gpt-5.6-sol")
    );
    assert_eq!(
        resolve_openai_alias("gpt-5.6-terra"),
        Some("openai.gpt-5.6-terra")
    );
    assert_eq!(
        resolve_openai_alias("gpt-5.6-luna"),
        Some("openai.gpt-5.6-luna")
    );
    assert_eq!(resolve_openai_alias("gpt-5.5"), Some("openai.gpt-5.5"));
    assert_eq!(resolve_openai_alias("gpt-5.4"), Some("openai.gpt-5.4"));
}

#[test]
fn full_ids_pass_through_resolve_model_id() {
    assert_eq!(
        BedrockProvider::resolve_model_id("gpt-6-astra"),
        "openai.gpt-6-astra"
    );
    assert_eq!(
        BedrockProvider::resolve_model_id("openai.gpt-6-astra"),
        "openai.gpt-6-astra"
    );
}

#[test]
fn non_openai_returns_none() {
    assert_eq!(resolve_openai_alias("claude-opus-4-7"), None);
    assert_eq!(resolve_openai_alias("nova-pro"), None);
}
