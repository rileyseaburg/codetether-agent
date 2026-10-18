//! Shared A2A prompt execution helpers.

use anyhow::Result;
use tokio::sync::mpsc;

use crate::Session;
use codetether_native_providers::ProviderRegistry;
use codetether_session_events::{SessionEvent, SessionResult};

/// Run a non-streaming A2A prompt with the cached provider registry.
pub async fn run<S: Session>(session: &mut S, prompt: &str) -> Result<SessionResult> {
    let registry = ProviderRegistry::shared_from_vault().await?;
    let (event_tx, event_rx) = mpsc::channel::<SessionEvent<S>>(1);
    drop(event_rx);
    session.prompt_with_events(prompt, event_tx, registry).await
}
