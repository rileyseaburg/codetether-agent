use super::spawn;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

#[tokio::test]
async fn drop_stops_heartbeat_and_releases_sender() {
    let (tx, mut rx) = mpsc::channel(1);
    let heartbeat = spawn(&tx, "call", "tool", Instant::now());
    drop(tx);
    drop(heartbeat);
    let closed = tokio::time::timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(matches!(closed, Ok(None)));
}
