//! Pre-dispatch rejections never claim that native actions were attempted.
use super::super::{execute, framing::REQUEST_LIMIT, queue};
use std::time::Duration;

#[tokio::test]
async fn oversized_request_is_rejected_before_spawn() {
    let input = serde_json::from_value(serde_json::json!({
        "action": "type_text", "text": "x".repeat(REQUEST_LIMIT)
    }))
    .unwrap();
    let result = execute(input).await.unwrap();
    assert!(!result.success);
    assert_eq!(
        result.metadata["error_code"],
        "COMPUTER_USE_WORKER_REQUEST_TOO_LARGE"
    );
    assert_eq!(result.metadata["action_effects_unknown"], false);
}

#[tokio::test]
async fn queue_acquisition_is_bounded_and_exclusive() {
    let guard = queue::acquire(Duration::from_secs(1)).await.unwrap();
    let error = match queue::acquire(Duration::from_millis(10)).await {
        Ok(_) => panic!("queue cannot be acquired twice"),
        Err(error) => error.result(),
    };
    assert_eq!(
        error.metadata["error_code"],
        "COMPUTER_USE_WORKER_QUEUE_TIMEOUT"
    );
    assert_eq!(error.metadata["action_effects_unknown"], false);
    drop(guard);
    assert!(queue::acquire(Duration::from_millis(10)).await.is_ok());
}
