//! Deterministic, no-I/O preflight gate regressions using Tokio's paused clock.
use super::{AuthFreshness, CHECK_TIMEOUT, TTL};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::time::{Duration, Instant, advance};

async fn counted(gate: &AuthFreshness, reads: &AtomicUsize) -> bool {
    gate.check(async {
        reads.fetch_add(1, Ordering::SeqCst);
        false
    })
    .await
}

mod concurrency;
mod freshness;
mod timeout;
