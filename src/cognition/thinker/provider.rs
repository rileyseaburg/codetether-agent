//! Cognition completion through an authenticated registry provider.

use anyhow::{Context, Result};

use super::{ThinkerConfig, ThinkerOutput, provider_output, provider_request};
use crate::provider::ProviderRegistry;

pub(super) async fn think(
    config: &ThinkerConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<ThinkerOutput> {
    let registry = ProviderRegistry::shared_from_vault().await?;
    let (provider, model) = registry.resolve_model(&config.model)?;
    provider_request::validate_model(&model)?;
    let request = provider_request::build(config, model, system_prompt, user_prompt);
    let response = provider.complete(request).await.with_context(|| {
        format!(
            "cognition provider '{}' rejected the request",
            provider.name()
        )
    })?;
    Ok(provider_output::convert(&config.model, response))
}
