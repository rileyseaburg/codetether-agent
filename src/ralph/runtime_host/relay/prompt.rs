//! Prompt a native session and retain the full result until the task joins.
use super::*;
pub(super) async fn run(
    mut session: Session,
    baton: String,
    event_tx: mpsc::Sender<SessionEvent>,
    registry: Arc<ProviderRegistry>,
) -> (Session, anyhow::Result<crate::session::SessionResult>) {
    let result = session.prompt_with_events(&baton, event_tx, registry).await;
    (session, result)
}
