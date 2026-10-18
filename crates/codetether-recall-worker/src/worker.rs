//! Background builder for coalesced recall snapshots.
use super::job;

pub(super) fn spawn<H: crate::Host>(session_id: String, host: std::sync::Arc<H>) {
    tokio::spawn(async move {
        loop {
            if let Some(session) = super::queue_state::take(&session_id) {
                process(session, host.as_ref()).await;
                continue;
            }
            if super::settle::settle(&session_id) {
                break;
            }
        }
    });
}

async fn process<H: crate::Host>(id: String, host: &H) {
    if let Err(error) = job::run(&id, host).await {
        tracing::warn!(target: "codetether_agent::session::recall::queue_worker",session_id = %id, %error, "incremental recall indexing failed");
    }
}
