//! `TCP_INFO` sampling for transport observability (Phase 4).
//!
//! Reads kernel TCP state via `getsockopt(SOL_TCP, TCP_INFO)` on a raw socket
//! fd, exposing RTT, cwnd, and retransmit counters so path degradation can be
//! distinguished from model latency. Linux-only; other platforms return `None`.
//! See `docs/transport-first-class-plan.md` Phase 4.

/// A snapshot of kernel TCP state for one connection.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::stream::tcp_info::TcpInfoSnapshot;
///
/// // All-zero snapshot is the documented "no data yet" baseline.
/// let snap = TcpInfoSnapshot::default();
/// assert_eq!(snap.rtt_us, 0);
/// assert_eq!(snap.total_retrans, 0);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TcpInfoSnapshot {
    /// Smoothed round-trip time in microseconds (`tcpi_rtt`).
    pub rtt_us: u32,
    /// RTT variance in microseconds (`tcpi_rttvar`).
    pub rttvar_us: u32,
    /// Send congestion window in segments (`tcpi_snd_cwnd`).
    pub snd_cwnd: u32,
    /// Currently-unacknowledged retransmits (`tcpi_retrans`).
    pub retrans: u32,
    /// Total retransmits over the connection lifetime (`tcpi_total_retrans`).
    pub total_retrans: u32,
}

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

/// Non-Linux platforms expose no `TCP_INFO`; `fd` is an opaque integer here.
#[cfg(not(target_os = "linux"))]
pub fn sample(_fd: i64) -> Option<TcpInfoSnapshot> {
    None
}

#[cfg(all(test, target_os = "linux"))]
#[path = "tcp_info_tests.rs"]
mod tests;
