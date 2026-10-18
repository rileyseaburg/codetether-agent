//! Resolve Morph requests through the existing cached provider registry.
use crate::{Application, ProviderRegistry};
use anyhow::{Context, Result};
/// Apply an edit through Morph using the application's shared provider registry.
/// # Arguments
/// `code` is original content; `instruction` and `update` describe the edit.
/// # Returns
/// Updated file content.
/// # Errors
/// Propagates registry, model resolution, completion, HTTP and response errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>() -> anyhow::Result<()> {
/// let _updated = codetether_application_providers::editing::apply_edit_with_morph::<H>(
///     "old", "replace old", "new",
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn apply_edit_with_morph<H: Application>(
    code: &str,
    instruction: &str,
    update: &str,
) -> Result<String> {
    codetether_morph_backend::apply_edit_with_morph(
        code,
        instruction,
        update,
        |model_ref| async move {
            let registry = ProviderRegistry::<H>::shared_from_vault()
                .await
                .context("Failed to initialize provider registry for Morph backend")?;
            let resolved = registry
                .resolve_model(&model_ref)
                .with_context(|| format!("Failed to resolve Morph model '{model_ref}'"))?;
            Ok(resolved)
        },
    )
    .await
}
