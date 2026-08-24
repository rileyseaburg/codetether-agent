//! Architecture-specific Linux seccomp syscall identities.

pub(super) fn audit_arch() -> Option<u32> {
    #[cfg(target_arch = "x86_64")]
    return Some(0xc000_003e);
    #[cfg(target_arch = "aarch64")]
    return Some(0xc000_00b7);
    #[allow(unreachable_code)]
    None
}

pub(super) fn denied(network: bool) -> Vec<u32> {
    let mut denied = privileged();
    if network {
        denied.extend(networking());
    }
    denied
}

fn privileged() -> Vec<u32> {
    [
        libc::SYS_ptrace, libc::SYS_mount, libc::SYS_umount2, libc::SYS_pivot_root,
        libc::SYS_chroot, libc::SYS_unshare, libc::SYS_setns,
        libc::SYS_bpf, libc::SYS_userfaultfd, libc::SYS_perf_event_open,
        libc::SYS_init_module, libc::SYS_finit_module, libc::SYS_delete_module,
        libc::SYS_kexec_load, libc::SYS_reboot, libc::SYS_open_by_handle_at,
        libc::SYS_io_uring_setup, libc::SYS_io_uring_enter,
        libc::SYS_io_uring_register,
    ].into_iter().map(|value| value as u32).collect()
}

#[cfg(test)]
#[path = "sandbox_seccomp_syscalls_tests.rs"]
mod tests;

fn networking() -> Vec<u32> {
    [
        libc::SYS_socket, libc::SYS_socketpair, libc::SYS_bind, libc::SYS_listen,
        libc::SYS_accept, libc::SYS_accept4, libc::SYS_connect, libc::SYS_sendto,
        libc::SYS_recvfrom, libc::SYS_sendmsg, libc::SYS_recvmsg, libc::SYS_sendmmsg,
        libc::SYS_recvmmsg, libc::SYS_setsockopt, libc::SYS_getsockopt, libc::SYS_shutdown,
    ].into_iter().map(|value| value as u32).collect()
}