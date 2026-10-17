//! Morph editing through a resolved provider or a direct OpenRouter endpoint.
//!
//! [apply_edit_with_morph] builds the existing editing prompt and selects the
//! configured transport. Callers supply provider resolution so credentials and
//! process-wide registry ownership stay outside this crate.
//! [should_use_morph_backend] exposes the explicit opt-in flag.
mod config;
mod direct;
mod direct_request;
mod direct_response;
mod provider;
mod provider_request;
mod provider_response;
mod response;
use anyhow::Result;
use codetether_provider_core::Provider;
use std::{future::Future, sync::Arc};
/// Read whether Morph-backed editing was explicitly enabled.
///
/// # Arguments
/// None.
/// # Returns
/// True only for the supported truthy opt-in values.
/// # Examples
/// ```rust
/// let _enabled = codetether_morph_backend::should_use_morph_backend();
/// ```
pub fn should_use_morph_backend() -> bool {
    config::backend_enabled()
}
/// Apply a Morph edit using the configured HTTP endpoint or supplied resolver.
///
/// # Arguments
/// * `code` - Original file content.
/// * `instruction` - Editing instructions.
/// * `update` - Replacement snippet or update context.
/// * `resolve` - Resolves the configured model to a provider and model name.
/// # Returns
/// The edited file content.
/// # Errors
/// Propagates provider resolution, HTTP, completion, and response errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _updated = codetether_morph_backend::apply_edit_with_morph(
///     "old", "replace old", "new", |_| async { anyhow::bail!("no provider configured") },
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn apply_edit_with_morph<F, R>(
    code: &str,
    instruction: &str,
    update: &str,
    resolve: F,
) -> Result<String>
where
    F: FnOnce(String) -> R + Send,
    R: Future<Output = Result<(Arc<dyn Provider>, String)>> + Send,
{
    let prompt = format!(
        "<instruction>{instruction}</instruction>\n<code>{code}</code>\n<update>{update}</update>"
    );
    if let Some(base_url) = config::direct_openrouter_base_url() {
        tracing::info!(target:"codetether_agent::tool::morph_backend",base_url=%base_url,"Using direct OpenRouter HTTP path for Morph backend");
        return direct::apply_with_direct_openrouter(&prompt, &base_url).await;
    }
    provider::apply_with_provider(&prompt, resolve).await
}

mod editor;
pub use editor::Editor;
