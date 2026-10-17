//! Kernel TCP statistics for one connection.

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
/// use codetether_telemetry::tcp_info::TcpInfoSnapshot;
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
