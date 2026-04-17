//! Instantiate a single provider from a Vault secret entry.
//!
//! Called once per provider found in Vault; returns `Some(Arc<dyn Provider>)`
//! when the provider was successfully created, `None` when the entry should be
//! skipped (missing required fields, unknown provider without a base_url, etc.).

use crate::provider::parse::normalize_minimax_anthropic_base_url;
use crate::provider::traits::Provider;
use crate::secrets::ProviderSecrets;
use std::sync::Arc;

macro_rules! try_register {
    ($expr:expr, $name:expr) => {
        match $expr {
            Ok(p) => return Some(Arc::new(p) as Arc<dyn Provider>),
            Err(e) => {
                tracing::warn!("Failed to init {}: {}", $name, e);
                return None;
            }
        }
    };
}

/// Given a `(provider_id, secrets)` pair from Vault, try to construct the
/// appropriate [`Provider`]. Returns `None` if the entry should be skipped.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::provider::init_dispatch::dispatch;
/// # fn demo(secrets: &codetether_agent::secrets::ProviderSecrets) {
/// let result = dispatch("openai", secrets);
/// // Some(Arc<dyn Provider>) or None
/// # }
/// ```
pub fn dispatch(provider_id: &str, secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    // ── Providers that need special handling (no api_key) ──────────

    if matches!(provider_id, "bedrock" | "aws-bedrock") {
        return super::init_dispatch_impl::dispatch_bedrock(secrets);
    }
    if matches!(provider_id, "vertex-glm" | "vertex-ai" | "gcp-glm") {
        return super::init_dispatch_impl::dispatch_vertex_glm(secrets);
    }
    if matches!(
        provider_id,
        "vertex-anthropic" | "vertex-claude" | "gcp-anthropic"
    ) {
        return super::init_dispatch_impl::dispatch_vertex_anthropic(secrets);
    }
    if matches!(provider_id, "openai-codex" | "codex" | "chatgpt") {
        return super::init_dispatch_impl::dispatch_codex(secrets);
    }
    if provider_id == "gemini-web" {
        return super::init_dispatch_impl::dispatch_gemini_web(secrets);
    }
    if matches!(provider_id, "local-cuda" | "local_cuda" | "localcuda") {
        return super::init_dispatch_impl::dispatch_local_cuda(secrets);
    }
    if provider_id == "huggingface" {
        return super::init_dispatch_impl::dispatch_huggingface(secrets);
    }

    // ── Providers that require an api_key ──────────────────────────

    let api_key = secrets.api_key.as_deref().filter(|k| !k.is_empty())?;

    match provider_id {
        "anthropic" | "anthropic-eu" | "anthropic-asia" => {
            let url = secrets
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.anthropic.com".into());
            try_register!(
                super::anthropic::AnthropicProvider::with_base_url(
                    api_key.into(),
                    url,
                    provider_id
                ),
                provider_id
            );
        }
        "google" | "google-vertex" => {
            try_register!(
                super::google::GoogleProvider::new(api_key.into()),
                provider_id
            );
        }
        "stepfun" => {
            try_register!(
                super::stepfun::StepFunProvider::new(api_key.into()),
                provider_id
            );
        }
        "openrouter" => {
            try_register!(
                super::openrouter::OpenRouterProvider::new(api_key.into()),
                provider_id
            );
        }
        "moonshotai" | "moonshotai-cn" => {
            try_register!(
                super::moonshot::MoonshotProvider::new(api_key.into()),
                provider_id
            );
        }
        "github-copilot" => {
            super::init_dispatch_impl::dispatch_copilot(api_key, secrets, "github-copilot")
        }
        "github-copilot-enterprise" => {
            super::init_dispatch_impl::dispatch_copilot_enterprise(api_key, secrets)
        }
        "glm5" | "glm-5-fp8" | "glm5-vastai" => {
            super::init_dispatch_impl::dispatch_glm5(api_key, secrets)
        }
        "zhipuai" | "zai" => {
            let url = secrets
                .base_url
                .clone()
                .unwrap_or_else(|| super::zai::DEFAULT_BASE_URL.into());
            try_register!(
                super::zai::ZaiProvider::with_base_url(api_key.into(), url),
                provider_id
            );
        }
        "cerebras" => {
            let url = secrets
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.cerebras.ai/v1".into());
            try_register!(
                super::openai::OpenAIProvider::with_base_url(api_key.into(), url, "cerebras"),
                provider_id
            );
        }
        "minimax" | "minimax-credits" => {
            let url = secrets
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.minimax.io/anthropic".into());
            let url = normalize_minimax_anthropic_base_url(&url);
            try_register!(
                super::anthropic::AnthropicProvider::with_base_url(
                    api_key.into(),
                    url,
                    provider_id
                ),
                provider_id
            );
        }
        "deepseek" | "groq" | "togetherai" | "fireworks-ai" | "mistral" | "nvidia" | "alibaba"
        | "openai" | "azure" | "novita" => {
            super::init_dispatch_impl::dispatch_openai_compat(api_key, secrets, provider_id)
        }
        _ => {
            // Unknown → try as OpenAI-compatible if we have a base_url
            if let Some(url) = &secrets.base_url {
                try_register!(
                    super::openai::OpenAIProvider::with_base_url(
                        api_key.into(),
                        url.clone(),
                        provider_id
                    ),
                    provider_id
                );
            }
            None
        }
    }
}
