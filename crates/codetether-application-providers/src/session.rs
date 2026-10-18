//! Construct provider registries from task claims stored in session metadata.
use crate::{Application, ProviderRegistry};
use codetether_session_metadata::SessionMetadata;

impl<H: Application> ProviderRegistry<H> {
    /// Load providers using the task-specific keys attached to a session.
    /// # Arguments
    /// `metadata` supplies the optional task-provider key claims.
    /// # Returns
    /// The registry selected by the existing task-key fallback policy.
    /// # Errors
    /// Propagates task-key decoding and configured provider loading failures.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_application_providers::Application>(metadata: &codetether_session_metadata::SessionMetadata) -> anyhow::Result<()> {
    /// let _registry = codetether_application_providers::ProviderRegistry::<H>::for_session(metadata).await?;
    /// # Ok(()) }
    /// ```
    pub async fn for_session(metadata: &SessionMetadata) -> anyhow::Result<Self> {
        Self::for_task_keys(metadata.provider_keys.as_ref()).await
    }
}
