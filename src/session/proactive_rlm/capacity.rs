//! Bounded global capacity for proactive RLM workers.

use std::sync::{Arc, OnceLock};

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

static CAPACITY: OnceLock<Arc<Semaphore>> = OnceLock::new();

pub(super) async fn acquire() -> Option<OwnedSemaphorePermit> {
    semaphore().acquire_owned().await.ok()
}

fn semaphore() -> Arc<Semaphore> {
    CAPACITY
        .get_or_init(|| Arc::new(Semaphore::new(max_jobs())))
        .clone()
}

fn max_jobs() -> usize {
    std::env::var("CODETETHER_RLM_PROACTIVE_MAX")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(2)
        .max(1)
}
