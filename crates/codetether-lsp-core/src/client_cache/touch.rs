//! Record client activity without changing its ownership identity.
use super::CacheClient;
use std::sync::Arc;
/// Update activity and return a clone of the same Arc.
///
/// # Arguments
/// * `client` — Cached client whose last-use time is refreshed.
/// # Returns
/// Another Arc pointing at the identical allocation.
/// Poisoned timestamp locks are recovered, preserving the existing policy.
pub fn touch<C: CacheClient>(client: &Arc<C>) -> Arc<C> {
    *client.last_used().lock().unwrap_or_else(|e| e.into_inner()) = std::time::Instant::now();
    Arc::clone(client)
}
