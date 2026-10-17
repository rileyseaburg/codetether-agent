//! Morph editing adapter using the application-owned provider registry.
use crate::provider::ProviderRegistry;
use anyhow::{Context, Result};
pub use codetether_morph_backend::should_use_morph_backend;
/// Apply an edit through Morph using the application's shared provider registry.
///
/// # Arguments
/// `code` is original content; `instruction` and `update` describe the edit.
/// # Returns
/// Updated file content.
/// # Errors
/// Propagates registry, completion, HTTP and response errors.
///
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _updated = codetether_agent::tool::morph_backend::apply_edit_with_morph(
///     "old", "replace old", "new",
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn apply_edit_with_morph(code: &str, instruction: &str, update: &str) -> Result<String> {
    codetether_morph_backend::apply_edit_with_morph(
        code,
        instruction,
        update,
        |model_ref| async move {
            let registry = ProviderRegistry::shared_from_vault()
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
