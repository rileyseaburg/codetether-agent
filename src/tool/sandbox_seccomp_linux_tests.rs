use super::prepare;
use std::os::fd::AsRawFd;

#[test]
fn prepare_returns_open_profile_fd() {
    let program = prepare(false).unwrap().unwrap();
    let flags = unsafe { libc::fcntl(program.file.as_raw_fd(), libc::F_GETFD) };
    assert!(flags >= 0);
    assert_eq!(flags & libc::FD_CLOEXEC, 0);
}
