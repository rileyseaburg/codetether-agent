//! Drain live session events into thread storage and optional JSONL.

use super::SharedMapper;
use crate::session::SessionEvent;
use crate::session::thread_store::{ThreadEvent, ThreadStore};
use anyhow::Result;
use tokio::sync::mpsc;

pub(super) async fn run(
    mut rx: mpsc::Receiver<SessionEvent>,
    mapper: SharedMapper,
    store: ThreadStore,
    jsonl: bool,
) -> Result<()> {
    while let Some(session_event) = rx.recv().await {
        let events = mapper.lock().await.map_session_event(&session_event);
        for event in events {
            emit(&store, &event, jsonl).await?;
        }
    }
    Ok(())
}

pub(super) async fn emit(store: &ThreadStore, event: &ThreadEvent, jsonl: bool) -> Result<()> {
    store.append(event.clone()).await?;
    if jsonl {
        crate::cli::run::jsonl::write_thread_event_to(std::io::stdout().lock(), event)?;
    }
    Ok(())
}
