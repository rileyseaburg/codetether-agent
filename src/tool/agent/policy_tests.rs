use super::*;
use crate::provider::ProviderRegistry;

#[test]
fn normalize_model_test() {
    assert_eq!(normalize_model("  ABC  "), "abc");
}

#[test]
fn free_model_detection() {
    assert!(is_free_model_id("z-ai:free"));
    assert!(is_free_model_id("model-free"));
    assert!(!is_free_model_id("gpt-4"));
}

#[tokio::test]
async fn included_provider_eligible() {
    let registry = ProviderRegistry::new();
    assert!(is_free_or_eligible("openai-codex/gpt-5-mini", &registry).await);
}

#[test]
fn openrouter_budget_allowlist_detection() {
    assert!(is_budget_allowlisted_model("openrouter", "qwen/qwen3.5-35ba3b"));
    assert!(!is_budget_allowlisted_model("openrouter", "qwen/qwen3-coder"));
    assert!(!is_budget_allowlisted_model("openai", "qwen/qwen3.5-35ba3b"));
}

#[tokio::test]
async fn openrouter_budget_allowlisted_model_is_eligible() {
    let registry = ProviderRegistry::new();
    assert!(is_free_or_eligible("openrouter/qwen/qwen3.5-35ba3b", &registry).await);
}
