//! Tests for the start/stop lifecycle.

use std::time::Duration;

use super::StartCognitionRequest;
use super::tests_request::create_in_swarm;
use super::tests_support::test_runtime;

#[tokio::test]
async fn start_stop_updates_runtime_status() {
    let runtime = test_runtime();
    runtime
        .start(Some(StartCognitionRequest {
            loop_interval_ms: Some(10),
            seed_persona: Some(create_in_swarm("seed", "watcher", "observe", "swarm-seed")),
        }))
        .await
        .expect("runtime should start");

    tokio::time::sleep(Duration::from_millis(60)).await;
    let running = runtime.status().await;
    assert!(running.running);
    assert!(running.events_buffered > 0);

    runtime
        .stop(Some("test".to_string()))
        .await
        .expect("runtime should stop");
    assert!(!runtime.status().await.running);
}
