//! Reconnect a failed or closed bus stream.
use super::*;
impl BusBridge {
    // ── internal ──────────────────────────────────────────────────────

    /// Background loop: connect to SSE, read envelopes, push to buffer.
    /// Reconnects on failure.
    pub(super) async fn reader_loop(&self) {
        loop {
            tracing::info!(target: "codetether_agent::mcp::bus_bridge", url = %self.bus_url, "BusBridge: connecting to bus SSE stream");
            match self.read_sse_stream().await {
                Ok(()) => {
                    tracing::info!(target: "codetether_agent::mcp::bus_bridge", "BusBridge: SSE stream closed normally");
                }
                Err(e) => {
                    tracing::warn!(target: "codetether_agent::mcp::bus_bridge", error = %e, "BusBridge: SSE stream error, reconnecting");
                }
            }
            self.connected.store(false, Ordering::Relaxed);
            tokio::time::sleep(RECONNECT_DELAY).await;
        }
    }
}
