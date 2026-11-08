//! FIFO ordering and endpoint sharing are deterministic without network sleeps.
use super::*;

#[tokio::test]
async fn lmstudio_queue_admits_waiters_in_fifo_order() {
    let gate = Gate::new(config(2));
    let first = gate.acquire().await.unwrap();
    let mut second = Box::pin(gate.acquire());
    let mut third = Box::pin(gate.acquire());
    assert!(poll!(second.as_mut()).is_pending());
    assert!(poll!(third.as_mut()).is_pending());
    drop(first);
    assert!(poll!(third.as_mut()).is_pending());
    let second = second.await.unwrap();
    assert!(poll!(third.as_mut()).is_pending());
    drop(second);
    assert!(third.await.is_ok());
}
