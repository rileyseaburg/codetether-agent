//! A simultaneous claim race has one winner and mutation-free conflicts.

use super::task;
use crate::server::task_queue::{ClaimError, KnativeTaskQueue};
use std::sync::Arc;
use tokio::{sync::Barrier, task::JoinSet};

#[tokio::test]
async fn sixteen_competing_claimants_have_exactly_one_winner() {
    let queue = KnativeTaskQueue::new();
    queue.push(task("pending")).await;
    let barrier = Arc::new(Barrier::new(16));
    let mut claimants = JoinSet::new();
    for _ in 0..16 {
        let queue = queue.clone();
        let barrier = barrier.clone();
        claimants.spawn(async move {
            barrier.wait().await;
            queue.claim("task-1").await
        });
    }
    let mut winners = 0;
    while let Some(result) = claimants.join_next().await {
        match result.expect("claimant task") {
            Ok(claimed) => {
                winners += 1;
                assert_eq!(claimed.status, "processing");
            }
            Err(reason) => assert_eq!(reason, ClaimError::NotPending),
        }
    }
    assert_eq!(winners, 1);
    assert_eq!(queue.get("task-1").await.unwrap().status, "processing");
}
