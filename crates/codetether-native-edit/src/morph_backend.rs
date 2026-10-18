//! Morph editing adapter using the application-owned provider registry.
use anyhow::Result;
pub use codetether_morph_backend::should_use_morph_backend;
use codetether_native_providers::ProviderRegistry;
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
/// let _updated = codetether_native_edit::morph_backend::apply_edit_with_morph(
///     "old", "replace old", "new",
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn apply_edit_with_morph(code: &str, instruction: &str, update: &str) -> Result<String> {
    codetether_application_providers::editing::apply_edit_with_morph::<
        codetether_native_providers::registry::application::Native,
    >(code, instruction, update)
    .await
}

/// Stateless Morph adapter sharing the application's provider registry.
///
/// # Examples
/// ```rust
/// let _editor = codetether_native_edit::morph_backend::ApplicationEditor;
/// ```
pub type ApplicationEditor = codetether_application_providers::editing::Editor<ProviderRegistry>;
/// Stateless value preserving the native unit-like editor construction.
#[allow(non_upper_case_globals)]
pub const ApplicationEditor: ApplicationEditor = ApplicationEditor::new();
