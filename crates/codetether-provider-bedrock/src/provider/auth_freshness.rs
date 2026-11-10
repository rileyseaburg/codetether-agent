//! Per-credential, clone-shared throttling of optional Vault preflights.
//! This caches check attempts, not credential validity: unchanged/unavailable
//! Vault and timeouts also get a bounded retry window. 401/403 recovery calls
//! the uncached adoption path directly and must never use this gate.
use std::{future::Future, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{Duration, Instant, timeout_at},
};

const TTL: Duration = Duration::from_secs(30);
const CHECK_TIMEOUT: Duration = Duration::from_secs(2);

/// Shared only by clones of one provider, never by unrelated credentials.
#[derive(Clone, Default)]
pub(in crate::provider) struct AuthFreshness(Arc<Mutex<Option<Instant>>>);

impl AuthFreshness {
    /// Run at most one check per TTL; report only an expired timeout.
    /// Lock wait and check execution share a single two-second deadline.
    pub(in crate::provider) async fn check(&self, check: impl Future<Output = bool>) -> bool {
        let deadline = Instant::now() + CHECK_TIMEOUT;
        let Ok(mut checked_at) = timeout_at(deadline, self.0.lock()).await else {
            return true;
        };
        if checked_at.is_some_and(|at| at.elapsed() < TTL) {
            return false;
        }
        let timed_out = timeout_at(deadline, check).await.is_err();
        // Record completion, not start, so waiting clones also skip a slow
        // attempt. Dropping/cancelling a check releases the mutex without
        // marking it fresh, allowing the next caller to retry.
        *checked_at = Some(Instant::now());
        timed_out
    }
}

#[cfg(test)]
mod tests;
