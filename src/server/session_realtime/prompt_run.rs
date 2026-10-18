//! Native session execution callback for the shared realtime turn loop.
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent, SessionResult};
use tokio::sync::mpsc;
#[path = "prompt_model.rs"]
mod prompt_model;
type PromptRun = codetether_session_realtime::PromptRun<Session>;

/// Start a prompt without blocking WebSocket steering input.
pub(super) fn start(session_id: String, message: String, model: Option<String>) -> PromptRun {
    let (event_tx, events) = mpsc::channel(256);
    let task = tokio::spawn(execute(session_id, message, model, event_tx));
    PromptRun { events, task }
}

/// Load, execute, and persist one session prompt.
async fn execute(
    session_id: String,
    message: String,
    model: Option<String>,
    event_tx: mpsc::Sender<SessionEvent>,
) -> Result<SessionResult, String> {
    let mut session = Session::load(&session_id)
        .await
        .map_err(|error| error.to_string())?;
    prompt_model::apply(&mut session.metadata, model);
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
