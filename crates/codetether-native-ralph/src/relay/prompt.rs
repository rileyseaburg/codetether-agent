//! Prompt a native session and retain the full result until the task joins.
use super::*;
pub(super) async fn run<S: Session>(
    mut session: S,
    baton: String,
    event_tx: mpsc::Sender<SessionEvent<S>>,
    registry: Arc<ProviderRegistry>,
) -> (S, anyhow::Result<SessionResult>) {
    let result = session.prompt_with_events(&baton, event_tx, registry).await;
    (session, result)
}
