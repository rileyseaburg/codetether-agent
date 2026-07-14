//! Controlling-terminal setup performed immediately before `exec`.

use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub(super) fn assign(process: &mut Command) {
    // SAFETY: setsid and ioctl are async-signal-safe operations before exec.
    unsafe {
        process.pre_exec(|| {
            if libc::setsid() == -1
                || libc::ioctl(std::io::stdin().as_raw_fd(), libc::TIOCSCTTY, 0) == -1
            {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
}
