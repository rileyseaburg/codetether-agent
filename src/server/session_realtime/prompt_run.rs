//! Ownership of a single event-producing session prompt task.

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent, SessionResult};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// Terminal join result produced by one prompt task.
pub(super) type PromptOutcome = Result<Result<SessionResult, String>, tokio::task::JoinError>;

/// Live event receiver and terminal task for one prompt.
pub(super) struct PromptRun {
    pub(super) events: mpsc::Receiver<SessionEvent>,
    pub(super) task: JoinHandle<Result<SessionResult, String>>,
}

impl PromptRun {
    /// Start a prompt without blocking WebSocket steering input.
    pub(super) fn start(session_id: String, message: String) -> Self {
        let (event_tx, events) = mpsc::channel(256);
        let task = tokio::spawn(execute(session_id, message, event_tx));
        Self { events, task }
    }
}

/// Load, execute, and persist one session prompt.
async fn execute(
    session_id: String,
    message: String,
    event_tx: mpsc::Sender<SessionEvent>,
) -> Result<SessionResult, String> {
    let mut session = Session::load(&session_id)
        .await
        .map_err(|error| error.to_string())?;
    let registry = ProviderRegistry::shared_from_vault()
        .await
        .map_err(|error| error.to_string())?;
    let result = session
        .prompt_with_events(&message, event_tx, registry)
        .await
        .map_err(|error| error.to_string())?;
    session.save().await.map_err(|error| error.to_string())?;
    Ok(result)
}
