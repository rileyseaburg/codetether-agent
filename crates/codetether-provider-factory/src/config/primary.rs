//! Existing primary provider configuration and its environment precedence.
use crate::{Registry, anthropic, google, openai};
use anyhow::Result;
use codetether_config::Config;
use std::sync::Arc;

pub(super) fn register(
    registry: &mut impl Registry,
    config: &Config,
    disable_env: bool,
) -> Result<()> {
    if let Some(pc) = config.providers.get("openai") {
        if let Some(key) = &pc.api_key {
            registry.register(Arc::new(openai::OpenAIProvider::new(key.clone())?));
        }
    } else if !disable_env && let Ok(key) = std::env::var("OPENAI_API_KEY") {
        registry.register(Arc::new(openai::OpenAIProvider::new(key)?));
    }
    if let Some(pc) = config.providers.get("anthropic") {
        if let Some(key) = &pc.api_key {
            let provider = if let Some(url) = pc.base_url.clone() {
                anthropic::AnthropicProvider::with_base_url(key.clone(), url, "anthropic")?
            } else {
                anthropic::AnthropicProvider::new(key.clone())?
            };
            registry.register(Arc::new(provider));
        }
    } else if !disable_env && let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        registry.register(Arc::new(anthropic::AnthropicProvider::new(key)?));
    }
    if let Some(pc) = config.providers.get("google") {
        if let Some(key) = &pc.api_key {
            registry.register(Arc::new(google::GoogleProvider::new(key.clone())?));
        }
    } else if !disable_env && let Ok(key) = std::env::var("GOOGLE_API_KEY") {
        registry.register(Arc::new(google::GoogleProvider::new(key)?));
    }
    Ok(())
}
