//! Serialize recall catalog mutations without blocking readers.

use std::sync::OnceLock;
use tokio::sync::{Mutex, MutexGuard};

/// Acquire the shared lock for sidecar and catalog mutations.
/// # Arguments
/// Takes no arguments.
/// # Returns
/// A guard releasing the lock when dropped.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let guard = codetether_recall_store::acquire().await;
/// drop(guard);
/// # }
/// ```
pub async fn acquire() -> MutexGuard<'static, ()> {
    mutation_lock().lock().await
}

fn mutation_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}
