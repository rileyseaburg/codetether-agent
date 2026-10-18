//! Resolve a cached application provider before the shared thinker completion.
use crate::{Application, ProviderRegistry};
use anyhow::Result;
use codetether_cognition_core::{ThinkerConfig, ThinkerOutput};

/// Complete a thought through the application's cached provider registry.
/// # Arguments
/// `config` selects the registered model; `system_prompt` and `user_prompt`
/// are passed unchanged to the shared thinker completion implementation.
/// # Returns
/// The provider completion converted to a structured thinker output.
/// # Errors
/// Propagates provider loading, model selection and completion errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>(config: &codetether_cognition_core::ThinkerConfig) -> anyhow::Result<()> {
/// let _thought = codetether_application_providers::thinker::think::<H>(config, "system", "user").await?;
/// # Ok(()) }
/// ```
pub async fn think<H: Application>(
    config: &ThinkerConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<ThinkerOutput> {
    let registry = ProviderRegistry::<H>::shared_from_vault().await?;
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
