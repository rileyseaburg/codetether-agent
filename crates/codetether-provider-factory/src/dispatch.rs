//! Instantiate a single provider from a Vault secret entry.
//!
//! Called once per provider found in Vault; returns `Some(Arc<dyn Provider>)`
//! when the provider was successfully created, `None` when the entry should be
//! skipped (missing required fields, unknown provider without a base_url, etc.).

use codetether_provider_anthropic::normalize_minimax_anthropic_base_url;
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
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
/// # Arguments
/// * `provider_id` — Configured provider name or supported alias.
/// * `secrets` — Credentials and endpoint/model overrides.
///
/// # Returns
/// A constructed provider, or `None` after a skipped or failed entry.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_provider_factory::dispatch;
/// # fn demo<H: codetether_provider_factory::Host>(secrets: &codetether_secrets::ProviderSecrets) {
/// let result = dispatch::<H>("openai", secrets);
/// // Some(Arc<dyn Provider>) or None
/// # }
/// ```
pub fn dispatch<H: crate::Host>(
    provider_id: &str,
    secrets: &ProviderSecrets,
) -> Option<Arc<dyn Provider>> {
    // ── Providers that need special handling (no api_key) ──────────

    if matches!(provider_id, "bedrock" | "aws-bedrock") {
        return crate::special::dispatch_bedrock::<H>(secrets);
    }
    if matches!(provider_id, "vertex-glm" | "vertex-ai" | "gcp-glm") {
        return crate::special::dispatch_vertex_glm(secrets);
    }
    if matches!(
        provider_id,
        "vertex-anthropic" | "vertex-claude" | "gcp-anthropic"
    ) {
        return crate::special::dispatch_vertex_anthropic(secrets);
    }
    if matches!(provider_id, "openai-codex" | "codex" | "chatgpt") {
        return crate::special::codex::dispatch(secrets);
    }
    if provider_id == "gemini-web" {
        return crate::special::dispatch_gemini_web(secrets);
    }
    if let Some(provider) = crate::models::dispatch::<H>(provider_id, secrets) {
        return provider;
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
                crate::anthropic::AnthropicProvider::with_base_url(
                    api_key.into(),
                    url,
                    provider_id
                ),
                provider_id
            );
        }
        "google" | "google-vertex" => {
            try_register!(
                crate::google::GoogleProvider::new(api_key.into()),
                provider_id
            );
        }
        "stepfun" => {
            try_register!(
                crate::stepfun::StepFunProvider::new(api_key.into()),
                provider_id
            );
        }
        "openrouter" => {
            try_register!(
                crate::openrouter::OpenRouterProvider::new(api_key.into()),
                provider_id
            );
        }
        "moonshotai" | "moonshotai-cn" => {
            try_register!(
                crate::moonshot::MoonshotProvider::new(api_key.into()),
                provider_id
            );
        }
        "github-copilot" => {
            crate::special::dispatch_copilot::<H>(api_key, secrets, "github-copilot")
        }
        "github-copilot-enterprise" => {
            crate::special::dispatch_copilot_enterprise::<H>(api_key, secrets)
        }
        "glm5" | "glm-5-fp8" | "glm5-vastai" => crate::special::dispatch_glm5(api_key, secrets),
        "zhipuai" | "zai" => {
            let url = secrets
                .base_url
                .clone()
                .unwrap_or_else(|| crate::zai::DEFAULT_BASE_URL.into());
            try_register!(
                crate::zai::ZaiProvider::with_base_url(api_key.into(), url),
                provider_id
            );
        }
        "cerebras" => {
            let url = secrets
                .base_url
                .clone()
                .unwrap_or_else(|| crate::cerebras::default_base_url().into());
            try_register!(crate::cerebras::new(api_key, Some(&url)), provider_id);
        }
        "usai" | "usai-gov" | "gsai" => {
            let url = secrets.base_url.clone().unwrap_or_else(|| {
                codetether_provider_tetherscript::usai::default_base_url().into()
            });
            try_register!(
                codetether_provider_tetherscript::usai::new(api_key, Some(&url)),
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
                crate::anthropic::AnthropicProvider::with_base_url(
                    api_key.into(),
                    url,
                    provider_id
                ),
                provider_id
            );
        }
        "deepseek" => {
            let url = secrets
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.deepseek.com".into());
            try_register!(
                crate::deepseek::DeepSeekProvider::with_base_url(api_key.into(), url),
                provider_id
            );
        }
        "groq" | "togetherai" | "fireworks-ai" | "mistral" | "nvidia" | "alibaba" | "openai"
        | "azure" | "novita" => {
            crate::special::dispatch_openai_compat(api_key, secrets, provider_id)
        }
        _ => {
            // Unknown → try as OpenAI-compatible if we have a base_url
            if let Some(url) = &secrets.base_url {
                try_register!(
                    crate::openai::OpenAIProvider::with_base_url(
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
