//! Build a [`ProviderRegistry`](super::ProviderRegistry) from the app [`Config`].

use super::anthropic;
use super::bedrock;
use super::fallback_policy;
use super::google;
use super::openai;
use super::registry::ProviderRegistry;
use anyhow::Result;
use std::sync::Arc;

impl ProviderRegistry {
    /// Initialize with default providers from the TOML config file.
    ///
    /// Reads `[providers.<name>]` sections and common `*_API_KEY` env vars.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::provider::ProviderRegistry;
    /// # async fn demo(cfg: &codetether_agent::config::Config) {
    /// let registry = ProviderRegistry::from_config(cfg).await.unwrap();
    /// # }
    /// ```
    pub async fn from_config(config: &crate::config::Config) -> Result<Self> {
        let mut registry = Self::new();

        // ---- OpenAI ----
        if let Some(pc) = config.providers.get("openai") {
            if let Some(key) = &pc.api_key {
                registry.register(Arc::new(openai::OpenAIProvider::new(key.clone())?));
            }
        } else if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            registry.register(Arc::new(openai::OpenAIProvider::new(key)?));
        }

        // ---- Anthropic ----
        if let Some(pc) = config.providers.get("anthropic") {
            if let Some(key) = &pc.api_key {
                let p = if let Some(url) = pc.base_url.clone() {
                    anthropic::AnthropicProvider::with_base_url(key.clone(), url, "anthropic")?
                } else {
                    anthropic::AnthropicProvider::new(key.clone())?
                };
                registry.register(Arc::new(p));
            }
        } else if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            registry.register(Arc::new(anthropic::AnthropicProvider::new(key)?));
        }

        // ---- Google ----
        if let Some(pc) = config.providers.get("google") {
            if let Some(key) = &pc.api_key {
                registry.register(Arc::new(google::GoogleProvider::new(key.clone())?));
            }
        } else if let Ok(key) = std::env::var("GOOGLE_API_KEY") {
            registry.register(Arc::new(google::GoogleProvider::new(key)?));
        }

        // ---- Novita (OpenAI-compatible) ----
        if let Some(pc) = config.providers.get("novita")
            && let Some(key) = &pc.api_key
        {
            let url = pc
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.novita.ai/openai/v1".into());
            registry.register(Arc::new(openai::OpenAIProvider::with_base_url(
                key.clone(),
                url,
                "novita",
            )?));
        }

        // ---- Bedrock (auto-detect from env / ~/.aws) ----
        if let Some(creds) = bedrock::AwsCredentials::from_environment() {
            let region = bedrock::AwsCredentials::detect_region()
                .unwrap_or_else(|| bedrock::DEFAULT_REGION.to_string());
            match bedrock::BedrockProvider::with_credentials(creds, region) {
                Ok(p) => registry.register(Arc::new(p)),
                Err(e) => tracing::warn!("Failed to init bedrock from AWS credentials: {}", e),
            }
        }

        // ---- Env-var fallback ----
        let disable = fallback_policy::env_fallback_disabled();
        if !disable {
            super::init_env::register_env_fallbacks(&mut registry);
        } else {
            tracing::info!(
                env = fallback_policy::DISABLE_ENV_FALLBACK,
                "Environment variable fallback disabled"
            );
        }

        Ok(registry)
    }
}
