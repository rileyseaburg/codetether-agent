//! Role adaptation through both transports, with model/provider isolation.
use super::*;
mod assertions;
mod history;
mod roundtrip;

#[tokio::test]
async fn groq_qwen_roles_and_other_models_are_isolated() {
    for (provider, model, expected) in [
        ("groq", "qwen/qwen3.8-27b", "system"),
        ("groq", "groq/qwen/qwen3.8-27b", "system"),
        ("groq", "qwen/qwen3.6-27b", "system"),
        ("groq", "openai/gpt-oss-20b", "developer"),
        ("openai", "qwen/qwen3.8-27b", "developer"),
    ] {
        roundtrip::check(provider, model, expected).await;
    }
}
