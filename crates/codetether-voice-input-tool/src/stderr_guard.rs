//! Serialize temporary ALSA diagnostic suppression around audio operations.

#[cfg(target_os = "linux")]
use super::stderr_redirect::StderrRedirectGuard;

/// Run `f` while temporarily redirecting process stderr to `/dev/null`.
///
/// Uses an RAII guard to ensure stderr is restored even if the closure
/// panics. A global mutex serializes access since this mutates
/// process-wide stderr.
#[cfg(target_os = "linux")]
pub(crate) fn silence_alsa<T>(f: impl FnOnce() -> T) -> T {
    use std::sync::{Mutex, OnceLock};

    static STDERR_REDIRECT_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let lock = STDERR_REDIRECT_LOCK.get_or_init(|| Mutex::new(()));
    let _lock_guard = lock.lock().unwrap_or_else(|p| p.into_inner());

    let Some(_redirect_guard) = StderrRedirectGuard::new() else {
        return f();
    };

    f()
}

/// Non-Linux backends do not emit ALSA diagnostics.
#[cfg(not(target_os = "linux"))]
pub(crate) fn silence_alsa<T>(f: impl FnOnce() -> T) -> T {
    f()
}
