//! Explicit configuration for hosted OpenAI-compatible providers.
use crate::{Registry, openai};
use anyhow::Result;
use codetether_config::Config;
use std::sync::Arc;

pub(super) fn register(registry: &mut impl Registry, config: &Config) -> Result<()> {
    if let Some(pc) = config.providers.get("groq") {
        if let Some(key) = &pc.api_key {
            registry.register(crate::groq::provider(key, pc.base_url.as_deref())?);
        }
    }
    if let Some(pc) = config.providers.get("novita") {
        if let Some(key) = &pc.api_key {
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
    }
    for provider_name in ["usai", "usai-gov", "gsai"] {
        if let Some(pc) = config.providers.get(provider_name) {
            if let Some(key) = &pc.api_key {
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
    }
    Ok(())
}
