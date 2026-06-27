//! Process-wide latest transport (TCP_INFO) snapshot.
//!
//! Holds the most recent [`TcpInfoSnapshot`] sampled from a probe connection to
//! the worker's server, so the TUI/telemetry can surface RTT, cwnd, and
//! retransmits as a first-class transport-health reading. See
//! `docs/transport-first-class-plan.md` Phase 4.

use std::sync::RwLock;

use crate::a2a::stream::tcp_info::TcpInfoSnapshot;

/// Thread-safe holder for the latest transport snapshot.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::transport::TransportMetrics;
/// use codetether_agent::a2a::stream::tcp_info::TcpInfoSnapshot;
///
/// let metrics = TransportMetrics::new();
/// assert!(metrics.latest().is_none());
/// metrics.record(TcpInfoSnapshot { rtt_us: 1500, ..Default::default() });
/// assert_eq!(metrics.latest().unwrap().rtt_us, 1500);
/// ```
pub struct TransportMetrics {
    latest: RwLock<Option<TcpInfoSnapshot>>,
}

impl Default for TransportMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl TransportMetrics {
    /// Create an empty metrics holder.
    pub fn new() -> Self {
        Self {
            latest: RwLock::new(None),
        }
    }

    /// Record the latest transport snapshot, replacing any prior reading.
    pub fn record(&self, snapshot: TcpInfoSnapshot) {
        if let Ok(mut guard) = self.latest.write() {
            *guard = Some(snapshot);
        }
    }

    /// Return the most recent snapshot, or `None` if never sampled.
    pub fn latest(&self) -> Option<TcpInfoSnapshot> {
        self.latest.read().ok().and_then(|g| *g)
    }
}
