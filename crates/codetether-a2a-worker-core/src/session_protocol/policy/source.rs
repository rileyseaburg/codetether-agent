pub use crate::execution_policy::Registry;
use codetether_provider_core::Message;
use std::{future::Future, path::PathBuf};
/// Native session state used to prepare a standard worker run.
/// # Implementors
/// Adapters retain credential loading, transcript provenance and title generation.
pub trait Source: Send {
    /// Native provider registry returned by credential loading.
    type Registry: Registry;
    /// Load task-specific provider credentials.
    /// # Errors
    /// Propagates provider loading failures.
    fn load(&mut self) -> impl Future<Output = anyhow::Result<Self::Registry>> + Send;
    /// Stored model selector.
    fn model(&self) -> Option<&str>;
    /// Append input using delegated-message provenance.
    fn append(&mut self, message: Message);
    /// Whether the session still needs a title.
    fn needs_title(&self) -> bool;
    /// Generate a title with the loaded registry.
    /// # Errors
    /// Propagates title-generation failures.
    fn title(
        &mut self,
        registry: &Self::Registry,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Optional configured workspace directory.
    fn workspace(&self) -> Option<PathBuf>;
    /// Identifier included in the final result.
    fn session_id(&self) -> &str;
}
