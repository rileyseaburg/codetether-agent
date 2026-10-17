//! Platform-specific TCP observation.

use super::TcpInfoSnapshot;

/// Sample `TCP_INFO` for the given raw socket file descriptor.
///
/// Returns `Some(snapshot)` on Linux when the syscall succeeds, else `None`.
#[cfg(target_os = "linux")]
pub fn sample(fd: std::os::fd::RawFd) -> Option<TcpInfoSnapshot> {
    let mut info: libc::tcp_info = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of::<libc::tcp_info>() as libc::socklen_t;
    let rc = unsafe {
        libc::getsockopt(
            fd,
            libc::SOL_TCP,
            libc::TCP_INFO,
            (&mut info as *mut libc::tcp_info).cast(),
            &mut len,
        )
    };
    if rc != 0 {
        return None;
    }
    Some(TcpInfoSnapshot {
        rtt_us: info.tcpi_rtt,
        rttvar_us: info.tcpi_rttvar,
        snd_cwnd: info.tcpi_snd_cwnd,
        retrans: info.tcpi_retrans,
        total_retrans: info.tcpi_total_retrans,
    })
}
