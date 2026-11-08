//! Admission, FIFO order, cancellation and timeout without upstream inference.
use super::*;
use futures::poll;
use std::time::Duration;
fn config(waiting: usize) -> Config {
    Config {
        output: 4096,
        waiting,
        wait: Duration::from_millis(40),
        generation: Duration::from_secs(1),
    }
}
#[tokio::test]
async fn lmstudio_queue_is_bounded_and_cancelled_waiter_releases_admission() {
    let gate = Gate::new(config(1));
    let first = gate.acquire().await.unwrap();
    let mut second = Box::pin(gate.acquire());
    assert!(poll!(second.as_mut()).is_pending());
    assert!(
        gate.acquire()
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("queue full")
    );
    drop(second);
    let mut replacement = Box::pin(gate.acquire());
    assert!(poll!(replacement.as_mut()).is_pending());
    drop(first);
    assert!(replacement.await.is_ok());
}
#[tokio::test]
async fn lmstudio_queue_wait_timeout_does_not_leak_capacity() {
    let gate = Gate::new(config(1));
    let first = gate.acquire().await.unwrap();
    assert!(
        gate.acquire()
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("wait timed out")
    );
    assert_eq!(gate.admission.available_permits(), 1);
    drop(first);
    assert!(gate.acquire().await.is_ok());
}
mod fifo;
