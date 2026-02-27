//! MCP-to-Bus bridge
//!
//! Connects to the HTTP server's `/v1/bus/stream` SSE endpoint and buffers
//! recent [`BusEnvelope`]s in a ring buffer.  The MCP server exposes this
//! data through tools (`bus_events`, `bus_status`) and resources
//! (`codetether://bus/events/recent`).
//!
//! The bridge runs as a background tokio task and reconnects automatically
//! on transient failures.

use crate::bus::BusEnvelope;
use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Maximum number of envelopes kept in the ring buffer.
const DEFAULT_BUFFER_SIZE: usize = 1_000;

/// Reconnect delay after a transient SSE failure.
const RECONNECT_DELAY: std::time::Duration = std::time::Duration::from_secs(3);

// ─── BusBridge ───────────────────────────────────────────────────────────

/// A read-only bridge from the HTTP bus SSE stream into the MCP process.
///
/// Call [`BusBridge::spawn`] to start the background reader, then query via
/// [`BusBridge::recent_events`] or [`BusBridge::status`].
#[derive(Debug)]
pub struct BusBridge {
    /// Ring buffer of recent envelopes (newest last).
    buffer: Arc<RwLock<VecDeque<BusEnvelope>>>,
    /// Whether the SSE reader is currently connected.
    connected: Arc<AtomicBool>,
    /// Total number of envelopes received since start.
    total_received: Arc<AtomicU64>,
    /// URL we connect to.
    bus_url: String,
    /// Max buffer capacity.
    capacity: usize,
}

impl BusBridge {
    /// Create a new bridge (does **not** start the background task).
    pub fn new(bus_url: String) -> Self {
        Self {
            buffer: Arc::new(RwLock::new(VecDeque::with_capacity(DEFAULT_BUFFER_SIZE))),
            connected: Arc::new(AtomicBool::new(false)),
            total_received: Arc::new(AtomicU64::new(0)),
            bus_url,
            capacity: DEFAULT_BUFFER_SIZE,
        }
    }

    /// Spawn the SSE reader as a background tokio task.
    ///
    /// Returns `Self` wrapped in an `Arc` for sharing with tool handlers.
    pub fn spawn(self) -> Arc<Self> {
        let bridge = Arc::new(self);
        let bg = Arc::clone(&bridge);
        tokio::spawn(async move {
            bg.reader_loop().await;
        });
        bridge
    }

    /// Query recent events with optional topic filter and limit.
    pub async fn recent_events(
        &self,
        topic_filter: Option<&str>,
        limit: usize,
        since: Option<DateTime<Utc>>,
    ) -> Vec<BusEnvelope> {
        let buf = self.buffer.read().await;
        buf.iter()
            .rev()
            .filter(|env| {
                if let Some(filter) = topic_filter {
                    topic_matches(&env.topic, filter)
                } else {
                    true
                }
            })
            .filter(|env| {
                if let Some(ts) = since {
                    env.timestamp >= ts
                } else {
                    true
                }
            })
            .take(limit)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev() // restore chronological order
            .collect()
    }

    /// Current bridge status summary (JSON-friendly).
    pub fn status(&self) -> BusBridgeStatus {
        BusBridgeStatus {
            connected: self.connected.load(Ordering::Relaxed),
            total_received: self.total_received.load(Ordering::Relaxed),
            bus_url: self.bus_url.clone(),
            buffer_capacity: self.capacity,
        }
    }

    /// Buffer size (number of envelopes currently held).
    pub async fn buffer_len(&self) -> usize {
        self.buffer.read().await.len()
    }

    // ── internal ──────────────────────────────────────────────────────

    /// Background loop: connect to SSE, read envelopes, push to buffer.
    /// Reconnects on failure.
    async fn reader_loop(&self) {
        loop {
            info!(url = %self.bus_url, "BusBridge: connecting to bus SSE stream");
            match self.read_sse_stream().await {
                Ok(()) => {
                    info!("BusBridge: SSE stream closed normally");
                }
                Err(e) => {
                    warn!(error = %e, "BusBridge: SSE stream error, reconnecting");
                }
            }
            self.connected.store(false, Ordering::Relaxed);
            tokio::time::sleep(RECONNECT_DELAY).await;
        }
    }

    /// Single SSE connection attempt.  Reads until the stream closes or errors.
    async fn read_sse_stream(&self) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let resp = client
            .get(&self.bus_url)
            .header("Accept", "text/event-stream")
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("SSE endpoint returned {}", resp.status());
        }

        self.connected.store(true, Ordering::Relaxed);
        info!("BusBridge: connected to SSE stream");

        // Read line-by-line.  SSE format:
        //   event: <type>\n
        //   data: <json>\n
        //   \n
        let mut event_type = String::new();
        let mut data_buf = String::new();

        use futures::StreamExt;
        let mut byte_stream = resp.bytes_stream();

        // Accumulate raw bytes into lines
        let mut line_buf = String::new();

        while let Some(chunk) = byte_stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);

            for ch in text.chars() {
                if ch == '\n' {
                    let line = std::mem::take(&mut line_buf);
                    self.process_sse_line(&line, &mut event_type, &mut data_buf)
                        .await;
                } else {
                    line_buf.push(ch);
                }
            }
        }

        Ok(())
    }

    /// Process a single SSE text line.
    async fn process_sse_line(&self, line: &str, event_type: &mut String, data_buf: &mut String) {
        if line.is_empty() {
            // Empty line = end of event
            if event_type == "bus" && !data_buf.is_empty() {
                match serde_json::from_str::<BusEnvelope>(data_buf) {
                    Ok(envelope) => {
                        self.push_envelope(envelope).await;
                    }
                    Err(e) => {
                        debug!(error = %e, "BusBridge: failed to parse bus envelope");
                    }
                }
            }
            event_type.clear();
            data_buf.clear();
        } else if let Some(rest) = line.strip_prefix("event:") {
            *event_type = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("data:") {
            if !data_buf.is_empty() {
                data_buf.push('\n');
            }
            data_buf.push_str(rest.trim());
        }
        // Ignore comment lines (`:`) and other prefixes
    }

    /// Push an envelope into the ring buffer, evicting oldest if full.
    async fn push_envelope(&self, envelope: BusEnvelope) {
        let mut buf = self.buffer.write().await;
        if buf.len() >= self.capacity {
            buf.pop_front();
        }
        buf.push_back(envelope);
        drop(buf);
        self.total_received.fetch_add(1, Ordering::Relaxed);
    }
}

/// Status snapshot returned by [`BusBridge::status`].
#[derive(Debug, Clone, serde::Serialize)]
pub struct BusBridgeStatus {
    pub connected: bool,
    pub total_received: u64,
    pub bus_url: String,
    pub buffer_capacity: usize,
}

// ─── Helpers ─────────────────────────────────────────────────────────────

/// Topic pattern matching (mirrors `server/mod.rs::topic_matches`).
fn topic_matches(topic: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix(".*") {
        return topic.starts_with(prefix);
    }
    if let Some(suffix) = pattern.strip_prefix(".*") {
        return topic.ends_with(suffix);
    }
    topic == pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_matches() {
        assert!(topic_matches("agent.123.events", "*"));
        assert!(topic_matches("agent.123.events", "agent.*"));
        assert!(topic_matches("ralph.prd1", "ralph.*"));
        assert!(!topic_matches("task.42", "agent.*"));
        assert!(topic_matches("agent.123.events", "agent.123.events"));
    }
}
