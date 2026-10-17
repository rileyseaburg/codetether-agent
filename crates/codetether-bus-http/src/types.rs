//! Bridge state and status contracts.
use super::*;

/// Status snapshot returned by [`BusBridge::status`].
///
/// # Examples
/// ```
/// let status = codetether_bus_http::BusBridgeStatus {
///     connected: false, total_received: 0, bus_url: "http://localhost".into(),
///     buffer_capacity: 1000,
/// };
/// assert!(!status.connected);
/// ```
#[derive(Debug, Clone, serde::Serialize)]
pub struct BusBridgeStatus {
    pub connected: bool,
    pub total_received: u64,
    pub bus_url: String,
    pub buffer_capacity: usize,
}

// ─── BusBridge ───────────────────────────────────────────────────────────

/// A read-only bridge from the HTTP bus SSE stream into the MCP process.
///
/// Call [`BusBridge::spawn`] to start the background reader, then query via
/// [`BusBridge::recent_events`] or [`BusBridge::status`].
///
/// # Examples
/// ```
/// let bridge = codetether_bus_http::BusBridge::new("http://localhost/v1/bus/stream".into());
/// assert_eq!(bridge.status().total_received, 0);
/// ```
#[derive(Debug)]
pub struct BusBridge {
    /// Ring buffer of recent envelopes (newest last).
    pub(super) buffer: Arc<RwLock<VecDeque<BusEnvelope>>>,
    /// Whether the SSE reader is currently connected.
    pub(super) connected: Arc<AtomicBool>,
    /// Total number of envelopes received since start.
    pub(super) total_received: Arc<AtomicU64>,
    /// URL we connect to.
    pub(super) bus_url: String,
    /// Optional bearer token for authenticated bus endpoints.
    pub(super) auth_token: Option<String>,
    /// Max buffer capacity.
    pub(super) capacity: usize,
}
