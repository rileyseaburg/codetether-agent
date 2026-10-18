//! Application session operations required by native A2A execution.
use codetether_native_providers::ProviderRegistry;
use codetether_session_events::{SessionEvent, SessionResult};
use std::{future::Future, sync::Arc};
use tokio::sync::mpsc;
/// Session persistence, model selection and prompt execution for A2A requests.
/// # Implementors
/// Applications preserve their normal session resolution, save and prompt paths.
pub trait Session: Send + Sync + Sized + 'static {
    /// Resolve or create the session associated with an optional context identity.
    /// # Errors
    /// Returns application session-resolution errors.
    fn resolve(context_id: Option<&str>) -> impl Future<Output = anyhow::Result<Self>> + Send;
    /// Stable session identity used by persistence diagnostics.
    fn id(&self) -> &str;
    /// Mutable model selection used by the configured A2A default.
    fn model_mut(&mut self) -> &mut Option<String>;
    /// Save through the application's existing persistence path.
    /// # Errors
    /// Returns session-persistence errors.
    fn save(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Run a prompt with the supplied event channel and shared provider registry.
    /// # Errors
    /// Returns prompt-execution errors.
    fn prompt_with_events(
        &mut self,
        prompt: &str,
        sender: mpsc::Sender<SessionEvent<Self>>,
        registry: Arc<ProviderRegistry>,
    ) -> impl Future<Output = anyhow::Result<SessionResult>> + Send;
}
