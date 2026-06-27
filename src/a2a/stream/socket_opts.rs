//! TCP socket-control options for the worker HTTP client (Phase 3).
//!
//! Reclaims the socket knobs that a bare `reqwest` client leaves at OS defaults.
//! Each value targets a specific failure mode observable from TCP/IP:
//! - `tcp_nodelay` disables Nagle so small framed SSE writes are not held back
//!   to coalesce with the next segment (Nagle ⊗ delayed-ACK ~40ms stalls).
//! - keepalive (idle/interval/retries) detects a silently dead peer instead of
//!   blocking forever on a half-open connection.
//! - `tcp_user_timeout` bounds how long unacknowledged data may stay in flight
//!   before the connection fails, instead of riding the full RTO backoff ladder
//!   on a black-holed path.
//!
//! See `docs/transport-first-class-plan.md` Phase 3.

use std::time::Duration;

use reqwest::ClientBuilder;

/// Disable Nagle's algorithm on the stream sockets.
pub const TCP_NODELAY: bool = true;
/// Idle time before the first keepalive probe is sent.
pub const KEEPALIVE_IDLE: Duration = Duration::from_secs(30);
/// Interval between keepalive probes once they start.
pub const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);
/// Number of unacknowledged keepalive probes before the connection is dropped.
pub const KEEPALIVE_RETRIES: u32 = 3;
/// Max time unacknowledged data may remain in flight before the conn fails.
pub const USER_TIMEOUT: Duration = Duration::from_secs(60);

/// Apply the worker's TCP socket-control options to a [`ClientBuilder`].
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::stream::socket_opts::apply_socket_opts;
///
/// let builder = reqwest::Client::builder();
/// let builder = apply_socket_opts(builder);
/// assert!(builder.build().is_ok());
/// ```
pub fn apply_socket_opts(builder: ClientBuilder) -> ClientBuilder {
    builder
        .tcp_nodelay(TCP_NODELAY)
        .tcp_keepalive(KEEPALIVE_IDLE)
        .tcp_keepalive_interval(KEEPALIVE_INTERVAL)
        .tcp_keepalive_retries(KEEPALIVE_RETRIES)
        .tcp_user_timeout(USER_TIMEOUT)
}

#[cfg(test)]
mod tests {
    use super::apply_socket_opts;

    #[test]
    fn applies_without_error() {
        let builder = apply_socket_opts(reqwest::Client::builder());
        assert!(builder.build().is_ok());
    }
}
