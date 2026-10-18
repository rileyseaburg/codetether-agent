//! Provider population from TOML configuration and permitted fallbacks.
use crate::{Registry, anthropic, fallback_policy, google, openai};
use anyhow::Result;
use codetether_bedrock_auth::{AwsCredentials, DEFAULT_REGION};
use std::sync::Arc;
/// Populate a registry from application configuration, preserving provider precedence.
///
/// # Arguments
/// * `registry` — Provider collection receiving configured providers.
/// * `config` — Application provider configuration.
///
/// # Returns
/// Success after configured providers and permitted fallbacks are registered.
///
/// # Errors
/// Returns the first configured provider constructor failure.
///
/// # Examples
/// ```no_run
/// # async fn load<H: codetether_provider_factory::Host>(registry: &mut impl codetether_provider_factory::Registry, config: &codetether_config::Config) -> anyhow::Result<()> {
/// codetether_provider_factory::config::populate_from_config::<H>(registry, config).await
/// # }
/// ```
pub async fn populate_from_config<H: crate::Host>(
    registry: &mut impl Registry,
    config: &codetether_config::Config,
) -> Result<()> {
    let disable_env = fallback_policy::env_fallback_disabled();

    // ---- OpenAI ----
    if let Some(pc) = config.providers.get("openai") {
        if let Some(key) = &pc.api_key {
            registry.register(Arc::new(openai::OpenAIProvider::new(key.clone())?));
        }
    } else if !disable_env && let Ok(key) = std::env::var("OPENAI_API_KEY") {
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
    } else if !disable_env && let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        registry.register(Arc::new(anthropic::AnthropicProvider::new(key)?));
    }

    // ---- Google ----
    if let Some(pc) = config.providers.get("google") {
        if let Some(key) = &pc.api_key {
            registry.register(Arc::new(google::GoogleProvider::new(key.clone())?));
        }
    } else if !disable_env && let Ok(key) = std::env::var("GOOGLE_API_KEY") {
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

    // ---- USAi / GSAi (TetherScript OpenAI-compatible provider) ----
    for provider_name in ["usai", "usai-gov", "gsai"] {
        if let Some(pc) = config.providers.get(provider_name)
            && let Some(key) = &pc.api_key
        {
            let url = pc.base_url.clone().unwrap_or_else(|| {
                codetether_provider_tetherscript::usai::default_base_url().into()
            });
            registry.register(Arc::new(codetether_provider_tetherscript::usai::new(
                key,
                Some(&url),
            )?));
            break;
        }
    }

    // ---- Bedrock (auto-detect from env / ~/.aws) ----
    if !disable_env && let Some(creds) = AwsCredentials::from_environment() {
        let region = AwsCredentials::detect_region().unwrap_or_else(|| DEFAULT_REGION.to_string());
        match H::bedrock_with_credentials(creds, region) {
            Ok(p) => registry.register(p),
            Err(e) => tracing::warn!("Failed to init bedrock from AWS credentials: {}", e),
        }
    }

    // ---- Env-var fallback ----
    if !disable_env {
        crate::environment::register_env_fallbacks::<H>(registry);
    } else {
        tracing::info!(
            env = fallback_policy::DISABLE_ENV_FALLBACK,
            "Env/AWS fallback disabled"
        );
    }

    Ok(())
}
