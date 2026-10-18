//! Event-collector integration tests for live TUI observation.

use super::collect::collect_events_with_idle_timeout;
use super::live_trace;
use crate::session::SessionEvent;
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::test]
async fn text_is_watchable_before_the_agent_finishes() {
    live_trace::begin("watched", "inspect".into());
    let (tx, mut rx) = mpsc::channel(4);
    let task = tokio::spawn(async move {
        collect_events_with_idle_timeout("watched", &mut rx, Duration::from_secs(1)).await
    });

    tx.send(SessionEvent::TextChunk("working live".into()))
        .await
        .unwrap();
    tokio::task::yield_now().await;
    let trace = live_trace::snapshot("watched").unwrap();
    assert_eq!(trace.streaming_text.as_deref(), Some("working live"));

    tx.send(SessionEvent::Done).await.unwrap();
    assert!(!task.await.unwrap().timed_out);
    live_trace::clear("watched");
}
