//! Morph editing with application-owned provider resolution and cache identity.
//!
//! [`apply_edit_with_morph`] resolves the model through the shared provider
//! registry. [`Editor`] supplies the same behavior to file editing tools.
use crate::{Application, ProviderRegistry};
use std::marker::PhantomData;
mod request;
pub use request::apply_edit_with_morph;

/// Stateless Morph editor bound to an application's provider registry type.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_providers::Application>() {
/// let _editor = codetether_application_providers::editing::Editor::<
///     codetether_application_providers::ProviderRegistry<H>,
/// >::new();
/// # }
/// ```
pub struct Editor<R>(PhantomData<fn() -> R>);
impl<R> Editor<R> {
    /// Construct a stateless editor; provider loading occurs when applying an edit.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<R> Default for Editor<R> {
    fn default() -> Self {
        Self::new()
    }
}
#[async_trait::async_trait]
impl<H: Application> codetether_morph_backend::Editor for Editor<ProviderRegistry<H>> {
    async fn apply(&self, code: &str, instruction: &str, update: &str) -> anyhow::Result<String> {
        apply_edit_with_morph::<H>(code, instruction, update).await
    }
}
