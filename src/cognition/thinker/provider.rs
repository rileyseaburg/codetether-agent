//! Resolve an authenticated provider before shared thinker completion.
use super::{ThinkerConfig, ThinkerOutput};
use crate::provider::ProviderRegistry;
use anyhow::Result;

pub(super) async fn think(
    config: &ThinkerConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<ThinkerOutput> {
    let registry = ProviderRegistry::shared_from_vault().await?;
    let (provider, model) = registry.resolve_model(&config.model)?;
    codetether_cognition_core::thinker::provider::think(
        config,
        provider.as_ref(),
        model,
        system_prompt,
        user_prompt,
    )
    .await
}
