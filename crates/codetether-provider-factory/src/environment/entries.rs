//! Ordered environment credential constructors for hosted providers.
use super::{Ctor, bedrock_bearer, usai_base_url_from_env};
use std::sync::Arc;

pub(super) fn entries<H: crate::Host>() -> Vec<(&'static str, &'static str, Ctor)> {
    vec![
        ("openai", "OPENAI_API_KEY", |k| {
            Ok(Arc::new(crate::openai::OpenAIProvider::new(k)?))
        }),
        ("anthropic", "ANTHROPIC_API_KEY", |k| {
            Ok(Arc::new(crate::anthropic::AnthropicProvider::new(k)?))
        }),
        ("google", "GOOGLE_API_KEY", |k| {
            Ok(Arc::new(crate::google::GoogleProvider::new(k)?))
        }),
        ("openrouter", "OPENROUTER_API_KEY", |k| {
            Ok(Arc::new(crate::openrouter::OpenRouterProvider::new(k)?))
        }),
        ("bedrock", "AWS_BEARER_TOKEN_BEDROCK", bedrock_bearer::<H>),
        ("zai", "ZAI_API_KEY", |k| {
            Ok(Arc::new(crate::zai::ZaiProvider::with_base_url(
                k,
                crate::zai::DEFAULT_BASE_URL.into(),
            )?))
        }),
        ("github-copilot", "GITHUB_COPILOT_TOKEN", |t| {
            H::copilot_new(t)
        }),
        ("cerebras", "CEREBRAS_API_KEY", |k| {
            crate::cerebras::provider(&k, None)
        }),
        ("groq", "GROQ_API_KEY", |k| crate::groq::provider(&k, None)),
        ("usai", "USAI_API_KEY", |k| {
            let base_url = usai_base_url_from_env();
            codetether_provider_tetherscript::usai::provider(&k, base_url.as_deref())
        }),
    ]
}
