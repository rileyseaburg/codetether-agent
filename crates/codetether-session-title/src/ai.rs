//! Automatic title policy: local Bedrock titles and model-backed titles elsewhere.
use crate::{TitleSession, ai_model::title_from_model};
use anyhow::Result;
use codetether_provider_core::Provider;

/// Generate a title with the provider and model selected for this turn.
///
/// Existing titles are preserved. Bedrock uses the local heuristic so optional
/// naming never adds inference latency before the answer. Other providers keep
/// model-backed naming with heuristic fallback. Call [`crate::title_from_model`]
/// explicitly when a Bedrock AI-generated title is needed.
///
/// # Errors
///
/// Returns an error if heuristic title generation fails after the model
/// request fails or produces no usable title.
/// # Arguments
/// `session` supplies native fields; `provider` and `model` select title generation.
/// # Returns
/// Unit after storing a model title or applying the existing heuristic fallback.
/// # Examples
/// ```rust,no_run
/// # async fn example<S: codetether_session_title::TitleSession>(session: &mut S, provider: &dyn codetether_provider_core::Provider) -> anyhow::Result<()> {
/// codetether_session_title::generate_with(session, provider, "model").await?;
/// # Ok(()) }
/// ```
pub async fn generate_with<S: TitleSession>(
    session: &mut S,
    provider: &dyn Provider,
    model: &str,
) -> Result<()> {
    if session.title().is_some() {
        return Ok(());
    }
    if provider.name() == "bedrock" {
        return crate::lifecycle::generate(session).await;
    }
    let Some(seed) = crate::seed::first_user_text(session.messages()) else {
        return Ok(());
    };
    match title_from_model(provider, model, &seed).await {
        Ok(Some(title)) => crate::lifecycle::set(session, title),
        _ => crate::lifecycle::generate(session).await?,
    }
    Ok(())
}

#[cfg(test)]
#[path = "ai_tests/mod.rs"]
mod tests;
