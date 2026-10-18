//! Serialize tests that observe the process-global pending request order.
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub(super) fn lock() -> std::sync::MutexGuard<'static, ()> {
    LOCK.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
