//! Public prompt entry points that delegate into
//! [`helper::prompt`](super::helper::prompt) and
//! [`helper::prompt_events`](super::helper::prompt_events).

use std::sync::Arc;

use anyhow::Result;

use super::events::{SessionEvent, SessionResult};
use super::helper;
use super::types::{ImageAttachment, Session};

impl Session {
    /// Execute a prompt and return the final text answer (non-streaming).
    ///
    /// This loads the provider registry from Vault on every call, which is
    /// fine for one-shot CLI use but expensive for the TUI — the TUI uses
    /// [`Session::prompt_with_events`] with a shared registry instead.
    ///
    /// # Errors
    ///
    /// Returns an error if no providers are configured, the model cannot
    /// be resolved, or the agentic loop exhausts its retry budgets.
    pub async fn prompt(&mut self, message: &str) -> Result<SessionResult> {
        helper::prompt::run_prompt(self, message).await
    }

    /// Process a user message with real-time event streaming for UI
    /// updates.
    ///
    /// Accepts a pre-loaded
    /// [`ProviderRegistry`](crate::provider::ProviderRegistry) to avoid
    /// re-fetching secrets from Vault on every message.
    pub async fn prompt_with_events(
        &mut self,
        message: &str,
        event_tx: tokio::sync::mpsc::Sender<SessionEvent>,
        registry: Arc<crate::provider::ProviderRegistry>,
    ) -> Result<SessionResult> {
        helper::prompt_events::run_prompt_with_events(self, message, Vec::new(), event_tx, registry)
            .await
    }

    /// Execute a prompt with optional image attachments and stream events.
    ///
    /// Images must be base64-encoded data URLs.
    pub async fn prompt_with_events_and_images(
        &mut self,
        message: &str,
        images: Vec<ImageAttachment>,
        event_tx: tokio::sync::mpsc::Sender<SessionEvent>,
        registry: Arc<crate::provider::ProviderRegistry>,
    ) -> Result<SessionResult> {
        helper::prompt_events::run_prompt_with_events(self, message, images, event_tx, registry)
            .await
    }
}
