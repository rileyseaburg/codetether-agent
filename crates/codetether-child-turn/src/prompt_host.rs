//! Native provider registry and prompt execution used by child startup.
use crate::FinalizationHost;
use anyhow::Result;
use async_trait::async_trait;
use codetether_child_events::Host;
use codetether_collaboration_input::MessageImage;
use std::sync::Arc;
use tokio::sync::mpsc;
/// Existing session prompt methods and provider registry.
/// # Implementors
/// Applications preserve the native plain/image prompt methods and their errors.
#[async_trait]
pub trait PromptHost: FinalizationHost + Host + Send + Sync + 'static {
    /// Existing provider registry shared with the child task.
    type Registry: Send + Sync + 'static;
    /// Resolve the currently configured registry, preserving load errors.
    async fn registry() -> Result<Arc<Self::Registry>>;
    /// Run a native prompt and return the mutated session on success.
    /// None selects the plain prompt API; Some selects the image-capable API.
    async fn prompt(
        session: Self::Session,
        text: String,
        images: Option<Vec<MessageImage>>,
        tx: mpsc::Sender<Self::SourceEvent>,
        registry: Arc<Self::Registry>,
    ) -> Result<Self::Session>;
}
