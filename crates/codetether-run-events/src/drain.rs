//! Drain live session events into thread storage and optional JSONL.

use super::SharedMapper;
use anyhow::Result;
use codetether_session_events::SessionEvent;
use codetether_thread_store::{ThreadEvent, ThreadStore};
use tokio::sync::mpsc;

pub(super) async fn run<S: Send + 'static>(
    mut rx: mpsc::Receiver<SessionEvent<S>>,
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
        codetether_run_jsonl::write_thread_event_to(std::io::stdout().lock(), event)?;
    }
    Ok(())
}
