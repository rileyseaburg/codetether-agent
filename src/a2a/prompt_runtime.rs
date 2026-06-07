//! Shared A2A prompt execution helpers.

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent, SessionResult};

/// Run a non-streaming A2A prompt with the cached provider registry.
pub async fn run(session: &mut Session, prompt: &str) -> Result<SessionResult> {
    let registry = ProviderRegistry::shared_from_vault().await?;
    let (event_tx, event_rx) = mpsc::channel::<SessionEvent>(1);
    drop(event_rx);
    session.prompt_with_events(prompt, event_tx, registry).await
}
