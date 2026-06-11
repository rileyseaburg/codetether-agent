use anyhow::{Result, anyhow};
use std::os::fd::RawFd;

pub(super) fn make_inheritable(fd: RawFd) -> Result<()> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags < 0 {
        return Err(anyhow!("read seccomp fd flags failed"));
    }
    let next = flags & !libc::FD_CLOEXEC;
    if unsafe { libc::fcntl(fd, libc::F_SETFD, next) } < 0 {
        return Err(anyhow!("clear seccomp fd close-on-exec failed"));
    }
    Ok(())
}
