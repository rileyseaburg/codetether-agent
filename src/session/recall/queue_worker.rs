//! Background builder for coalesced recall snapshots.
mod job;

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

async fn process(id: String) {
    if let Err(error) = job::run(&id).await {
        tracing::warn!(session_id = %id, %error, "incremental recall indexing failed");
    }
}
