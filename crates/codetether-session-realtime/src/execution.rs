//! Owned realtime prompt task and ordered session lifecycle.
use crate::{Host, PromptRun, prompt_model};
use codetether_session_events::{SessionEvent, SessionResult};
use tokio::sync::mpsc;
/// Start a prompt task without blocking incoming WebSocket steering.
/// # Arguments
/// * `session_id` - Existing session identity to load.
/// * `message` - Prompt text supplied by the client.
/// * `model` - Optional explicit model selection.
/// # Returns
/// The bounded event receiver and owned, cancellable prompt task.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_session_realtime::Host>() {
/// let task = codetether_session_realtime::execution::start::<H>("id".into(), "hello".into(), None);
/// # }
/// ```
pub fn start<H: Host>(
    session_id: String,
    message: String,
    model: Option<String>,
) -> PromptRun<H::Session> {
    let (event_tx, events) = mpsc::channel(256);
    let task = tokio::spawn(execute::<H>(session_id, message, model, event_tx));
    PromptRun { events, task }
}

async fn execute<H: Host>(
    session_id: String,
    message: String,
    model: Option<String>,
    event_tx: mpsc::Sender<SessionEvent<H::Session>>,
) -> Result<SessionResult, String> {
    let mut session = H::load(&session_id)
        .await
        .map_err(|error| error.to_string())?;
    prompt_model::apply(H::metadata(&mut session), model);
    let registry = H::load_registry()
        .await
        .map_err(|error| error.to_string())?;
    let result = H::prompt(&mut session, &message, event_tx, registry)
        .await
        .map_err(|error| error.to_string())?;
    H::save(&session).await.map_err(|error| error.to_string())?;
    Ok(result)
}
