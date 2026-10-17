//! Complete a thought through an already resolved registry provider.
use super::protocol::{provider_output, provider_request};
use super::{ThinkerConfig, ThinkerOutput};
use anyhow::{Context, Result};
use codetether_provider_core::Provider;

/// Validate the resolved model, complete its request and convert the output.
///
/// # Arguments
/// * `config` - Settings and the configured model label for the output.
/// * `provider` - Already-resolved provider instance.
/// * `model` - Resolved model passed into the completion request.
/// * `system_prompt` - Caller system instructions.
/// * `user_prompt` - Caller user text.
///
/// # Returns
/// Normalized thinker output using the configured model label.
///
/// # Errors
/// Returns model validation or provider completion errors with existing context.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::thinker::{ThinkerConfig, provider::think};
/// # async fn example(config: &ThinkerConfig, provider: &dyn codetether_provider_core::Provider, model: String) {
/// let _output = think(config, provider, model, "Answer concisely", "Hello").await.unwrap();
/// # }
/// ```
pub async fn think(
    config: &ThinkerConfig,
    provider: &dyn Provider,
    model: String,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<ThinkerOutput> {
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
