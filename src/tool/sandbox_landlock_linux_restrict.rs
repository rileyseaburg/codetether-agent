pub(crate) fn restrict(fd: i32) -> std::io::Result<()> {
    let prctl = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
    let restricted = unsafe { libc::syscall(libc::SYS_landlock_restrict_self, fd, 0u32) };
    unsafe { libc::close(fd) };
    if prctl != 0 || restricted != 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub(crate) fn close_with_error(fd: i32) -> std::io::Result<()> {
    let error = std::io::Error::last_os_error();
    unsafe { libc::close(fd) };
    Err(error)
}
