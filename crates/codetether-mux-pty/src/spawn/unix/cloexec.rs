//! Close-on-exec for PTY descriptors so sibling children never inherit them.
//!
//! `openpty` returns descriptors without `FD_CLOEXEC`. Any process spawned
//! concurrently on another thread (a second session's shell, an agent task)
//! would otherwise inherit a copy of this PTY's slave, keeping the master's
//! read side open after the real child exits and never reporting EOF.

use std::fs::File;
use std::os::fd::AsRawFd;

use anyhow::{Result, bail};

pub(super) fn set(file: &File) -> Result<()> {
    let fd = file.as_raw_fd();
    // SAFETY: fcntl on a descriptor this process owns; flags are read then
    // written back with only FD_CLOEXEC added.
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags == -1 {
        bail!(std::io::Error::last_os_error());
    }
    if unsafe { libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC) } == -1 {
        bail!(std::io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::os::fd::AsRawFd;

    #[test]
    fn marks_the_descriptor_close_on_exec() {
        let file = std::fs::File::open("/dev/null").unwrap();
        super::set(&file).unwrap();
        let flags = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFD) };
        assert_ne!(flags & libc::FD_CLOEXEC, 0);
    }
}
