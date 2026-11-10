//! Cloned providers collapse simultaneous preflights into one async check.
use super::*;
use tokio::sync::oneshot;

#[tokio::test(start_paused = true)]
async fn concurrent_clones_share_one_in_flight_attempt() {
    let gate = AuthFreshness::default();
    let reads = Arc::new(AtomicUsize::new(0));
    let (started, observed) = oneshot::channel();
    let (release, ready) = oneshot::channel();
    let leader_gate = gate.clone();
    let leader_reads = reads.clone();
    let leader = tokio::spawn(async move {
        leader_gate
            .check(async {
                leader_reads.fetch_add(1, Ordering::SeqCst);
                started.send(()).unwrap();
                ready.await.unwrap();
                false
            })
            .await
    });
    observed.await.unwrap();
    let mut waiters = Vec::new();
    for _ in 0..16 {
        let gate = gate.clone();
        let reads = reads.clone();
        waiters.push(tokio::spawn(async move { counted(&gate, &reads).await }));
    }
    tokio::task::yield_now().await;
    assert_eq!(reads.load(Ordering::SeqCst), 1);
    release.send(()).unwrap();
    assert!(!leader.await.unwrap());
    for waiter in waiters {
        assert!(!waiter.await.unwrap());
    }
    assert_eq!(reads.load(Ordering::SeqCst), 1);
}
