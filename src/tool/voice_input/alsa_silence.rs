//! Linux ALSA stderr suppression for probing missing audio devices.

/// Run `f` while temporarily redirecting process stderr to `/dev/null`.
#[cfg(target_os = "linux")]
pub(crate) fn silence_alsa<T>(f: impl FnOnce() -> T) -> T {
    use std::fs::OpenOptions;
    use std::os::fd::AsRawFd;

    let Ok(devnull) = OpenOptions::new().write(true).open("/dev/null") else {
        return f();
    };
    unsafe {
        let saved = libc::dup(libc::STDERR_FILENO);
        if saved < 0 {
            return f();
        }
        libc::dup2(devnull.as_raw_fd(), libc::STDERR_FILENO);
        let result = f();
        libc::dup2(saved, libc::STDERR_FILENO);
        libc::close(saved);
        result
    }
}

/// Non-Linux backends do not emit ALSA diagnostics.
#[cfg(not(target_os = "linux"))]
pub(crate) fn silence_alsa<T>(f: impl FnOnce() -> T) -> T {
    f()
}
