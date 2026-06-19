//! In-memory ring buffer of recently published [`BusEnvelope`]s.
//!
//! `tokio::sync::broadcast` is live-only: anything published before a
//! subscriber attaches is lost. The recorder keeps the last `cap` envelopes so
//! agents and tools can inspect recent bus activity on demand without needing a
//! pre-attached subscriber or a durable sink.

use std::collections::VecDeque;
use std::sync::Mutex;

use super::BusEnvelope;

/// Bounded, thread-safe history of recent bus envelopes.
#[derive(Debug)]
pub struct BusRecorder {
    cap: usize,
    buf: Mutex<VecDeque<BusEnvelope>>,
}

impl BusRecorder {
    /// Create a recorder retaining at most `cap` envelopes.
    pub fn new(cap: usize) -> Self {
        Self {
            cap: cap.max(1),
            buf: Mutex::new(VecDeque::with_capacity(cap.max(1))),
        }
    }

    /// Append an envelope, evicting the oldest when at capacity.
    pub fn record(&self, envelope: &BusEnvelope) {
        let mut buf = match self.buf.lock() {
            Ok(b) => b,
            Err(p) => p.into_inner(),
        };
        if buf.len() == self.cap {
            buf.pop_front();
        }
        buf.push_back(envelope.clone());
    }

    /// Return up to `limit` most-recent envelopes (newest last), optionally
    /// filtered by a topic prefix.
    pub fn recent(&self, limit: usize, topic_prefix: Option<&str>) -> Vec<BusEnvelope> {
        let buf = match self.buf.lock() {
            Ok(b) => b,
            Err(p) => p.into_inner(),
        };
        let mut matched: Vec<BusEnvelope> = buf
            .iter()
            .filter(|e| topic_prefix.is_none_or(|p| e.topic.starts_with(p)))
            .cloned()
            .collect();
        if matched.len() > limit {
            matched.drain(0..matched.len() - limit);
        }
        matched
    }
}
