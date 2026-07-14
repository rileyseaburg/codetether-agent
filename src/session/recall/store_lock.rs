//! Serialize recall catalog mutations without blocking readers.

use std::sync::OnceLock;
use tokio::sync::{Mutex, MutexGuard};

pub(super) async fn acquire() -> MutexGuard<'static, ()> {
    mutation_lock().lock().await
}

fn mutation_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}
