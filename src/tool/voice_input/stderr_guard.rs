//! RAII guard for temporarily redirecting stderr to `/dev/null` on Linux.

use std::os::fd::RawFd;

/// Guard that restores the original stderr file descriptor on drop.
///
/// On creation, duplicates the current stderr fd, then redirects stderr
/// to `/dev/null`. When dropped (including during unwinding), the original
/// stderr is restored and the backup fd is closed.
pub(crate) struct StderrRedirectGuard {
    saved_stderr: RawFd,
}

impl StderrRedirectGuard {
    /// Create a new guard that silences stderr until dropped.
    ///
    /// Returns `None` if either `dup` or `dup2` fails, in which case
    /// stderr is left unchanged.
    #[cfg(target_os = "linux")]
    pub(crate) fn new() -> Option<Self> {
        use std::fs::OpenOptions;
        use std::os::fd::AsRawFd;

        let devnull = OpenOptions::new().write(true).open("/dev/null").ok()?;
        unsafe {
            let saved_stderr = libc::dup(libc::STDERR_FILENO);
            if saved_stderr < 0 {
                return None;
            }
            if libc::dup2(devnull.as_raw_fd(), libc::STDERR_FILENO) < 0 {
                libc::close(saved_stderr);
                return None;
            }
            Some(Self { saved_stderr })
        }
    }
}

impl Drop for StderrRedirectGuard {
    fn drop(&mut self) {
        unsafe {
            libc::dup2(self.saved_stderr, libc::STDERR_FILENO);
            libc::close(self.saved_stderr);
        }
    }
}

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
