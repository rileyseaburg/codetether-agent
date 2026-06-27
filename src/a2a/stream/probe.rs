//! Periodic transport-health probe (Phase 4 integration).
//!
//! reqwest hides the stream `RawFd`, so rather than fight its connector
//! abstraction we open a short-lived probe connection to the same server
//! `host:port`, sample `TCP_INFO` off that socket, and record it to
//! `TRANSPORT_METRICS`. This measures the same network path the worker stream
//! traverses. See `docs/transport-first-class-plan.md` Phase 4.

use std::time::Duration;

use tokio::net::TcpStream;

use crate::a2a::stream::tcp_info::{TcpInfoSnapshot, sample};

/// Connect to `addr`, sample `TCP_INFO`, and return the snapshot.
///
/// Returns `None` if the connection or syscall fails (non-Linux always `None`).
pub async fn probe_once(addr: &str) -> Option<TcpInfoSnapshot> {
    let stream = TcpStream::connect(addr).await.ok()?;
    #[cfg(target_os = "linux")]
    {
        use std::os::fd::AsRawFd;
        return sample(stream.as_raw_fd());
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (&stream, sample);
        None
    }
}

/// Parse a server base URL into a `host:port` probe target.
///
/// Falls back to default ports (443 for https, 80 otherwise) when absent.
pub fn probe_target(server_url: &str) -> Option<String> {
    let rest = server_url
        .strip_prefix("https://")
        .map(|r| (r, 443u16))
        .or_else(|| server_url.strip_prefix("http://").map(|r| (r, 80)))?;
    let (authority, default_port) = rest;
    let host_port = authority.split('/').next().unwrap_or(authority);
    if host_port.contains(':') {
        Some(host_port.to_string())
    } else {
        Some(format!("{host_port}:{default_port}"))
    }
}

/// Default interval between transport probes.
pub const PROBE_INTERVAL: Duration = Duration::from_secs(15);

#[cfg(test)]
mod tests {
    use super::probe_target;

    #[test]
    fn parses_targets_with_default_ports() {
        assert_eq!(probe_target("http://host/path").as_deref(), Some("host:80"));
        assert_eq!(probe_target("https://host").as_deref(), Some("host:443"));
        assert_eq!(
            probe_target("http://host:8001/x").as_deref(),
            Some("host:8001")
        );
        assert!(probe_target("ftp://host").is_none());
    }
}
