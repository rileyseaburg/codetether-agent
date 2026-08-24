//! Seccomp allows ordinary worker-thread creation.

use super::denied;

#[test]
fn clone3_is_not_singularly_blocked_while_clone_is_allowed() {
    let syscalls = denied(false);
    assert!(!syscalls.contains(&(libc::SYS_clone3 as u32)));
    assert!(!syscalls.contains(&(libc::SYS_clone as u32)));
    assert!(syscalls.contains(&(libc::SYS_unshare as u32)));
}