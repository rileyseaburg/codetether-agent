use super::super::{CoordinationReply, LeaseRegistry};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn release_wakes_a_waiting_acquisition() {
    let registry = Arc::new(LeaseRegistry::new());
    let root = std::path::Path::new("/workspace");
    registry.acquire("one", "a", root, vec!["src/a.rs".into()]);
    let waiting = Arc::clone(&registry);
    let task = tokio::spawn(async move {
        waiting
            .acquire_wait("two", "b", root, vec!["src/a.rs".into()], 1_000)
            .await
    });
    tokio::time::sleep(Duration::from_millis(20)).await;
    registry.release("one");
    let reply = task.await.unwrap();
    assert!(matches!(reply, CoordinationReply::Acquired { waited_ms, .. } if waited_ms < 1_000));
}

#[tokio::test]
async fn timeout_reports_how_long_the_coordinator_waited() {
    let registry = LeaseRegistry::new();
    let root = std::path::Path::new("/workspace");
    registry.acquire("one", "a", root, vec!["src/a.rs".into()]);
    let reply = registry
        .acquire_wait("two", "b", root, vec!["src/a.rs".into()], 20)
        .await;
    assert!(matches!(reply, CoordinationReply::Blocked { waited_ms, .. } if waited_ms >= 15));
}
