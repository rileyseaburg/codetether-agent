//! Background builder for coalesced recall snapshots.

pub(super) fn spawn(session_id: String) {
    tokio::spawn(async move {
        loop {
            if let Some(session) = super::queue::take(&session_id) {
                process(session).await;
                continue;
            }
            if super::queue::settle(&session_id) {
                break;
            }
        }
    });
}

async fn process(session: crate::session::Session) {
    let id = session.id.clone();
    let indexed = tokio::task::spawn_blocking(move || super::build::session(&session)).await;
    let Ok(Some(indexed)) = indexed else {
        tracing::warn!(session_id = %id, "recall sidecar build task failed");
        return;
    };
    if let Err(error) = super::store::upsert(indexed).await {
        tracing::warn!(session_id = %id, %error, "recall sidecar persistence failed");
    }
}
