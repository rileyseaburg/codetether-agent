//! Inactivity-timeout regression tests for sub-agent event collection.

use super::collect::collect_events_with_idle_timeout;
use crate::session::SessionEvent;
use std::time::Duration;
use tokio::sync::mpsc;

const TIMEOUT: Duration = Duration::from_secs(300);

#[tokio::test(start_paused = true)]
async fn activity_resets_timeout() {
    let (tx, mut rx) = mpsc::channel(4);
    let task =
        tokio::spawn(
            async move { collect_events_with_idle_timeout("active", &mut rx, TIMEOUT).await },
        );

    tokio::task::yield_now().await;
    tokio::time::advance(Duration::from_secs(299)).await;
    tx.send(SessionEvent::Thinking).await.unwrap();
    tokio::task::yield_now().await;
    tokio::time::advance(Duration::from_secs(299)).await;
    tx.send(SessionEvent::Done).await.unwrap();

    let state = task.await.unwrap();
    assert!(!state.timed_out);
    assert!(state.error.is_none());
}

#[tokio::test(start_paused = true)]
async fn inactivity_still_times_out() {
    let (_tx, mut rx) = mpsc::channel(1);
    let task = tokio::spawn(async move {
        collect_events_with_idle_timeout("inactive", &mut rx, TIMEOUT).await
    });

    tokio::task::yield_now().await;
    tokio::time::advance(TIMEOUT).await;
    let state = task.await.unwrap();

    assert!(state.timed_out);
    assert_eq!(
        state.error.as_deref(),
        Some("Agent timed out after 5 minutes of inactivity")
    );
}
