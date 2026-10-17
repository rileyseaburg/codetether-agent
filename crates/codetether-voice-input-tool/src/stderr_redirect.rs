//! Linux stderr descriptor restoration on normal exit and unwinding.
//! RAII guard for temporarily redirecting stderr to `/dev/null` on Linux.

/// Guard that restores the original stderr file descriptor on drop.
///
/// On creation, duplicates the current stderr fd, then redirects stderr
/// to `/dev/null`. When dropped (including during unwinding), the original
/// stderr is restored and the backup fd is closed.
#[cfg(target_os = "linux")]
pub(crate) struct StderrRedirectGuard {
    saved_stderr: std::os::fd::RawFd,
}

#[cfg(target_os = "linux")]
impl StderrRedirectGuard {
    /// Create a new guard that silences stderr until dropped.
    ///
    /// Returns `None` if either `dup` or `dup2` fails, in which case
    /// stderr is left unchanged.
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

#[cfg(target_os = "linux")]
impl Drop for StderrRedirectGuard {
    fn drop(&mut self) {
        unsafe {
            libc::dup2(self.saved_stderr, libc::STDERR_FILENO);
            libc::close(self.saved_stderr);
        }
    }
}
