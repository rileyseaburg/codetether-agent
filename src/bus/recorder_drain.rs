//! Lossless cursor drain over [`BusRecorder`](super::BusRecorder).
//!
//! Unlike the live `broadcast` receiver (which silently skips on lag), this
//! lets a consumer track a monotonic `pushed` cursor and pull every envelope
//! recorded since its last drain. If the consumer falls behind the ring
//! capacity, only the evicted-oldest are missed; nothing within the retained
//! window is dropped.

use std::sync::atomic::Ordering;

use super::{BusEnvelope, BusRecorder};

impl BusRecorder {
    /// Current monotonic count of envelopes ever recorded.
    pub fn cursor(&self) -> u64 {
        self.pushed.load(Ordering::Relaxed)
    }

    /// Drain all envelopes recorded since `cursor`, returning them (oldest
    /// first) and the new cursor to pass next time.
    pub fn drain_since(&self, cursor: u64) -> (Vec<BusEnvelope>, u64) {
        let buf = match self.buf.lock() {
            Ok(b) => b,
            Err(p) => p.into_inner(),
        };
        let pushed = self.pushed.load(Ordering::Relaxed);
        if pushed <= cursor {
            return (Vec::new(), pushed);
        }
        let new = pushed - cursor;
        let take = (new as usize).min(buf.len());
        let start = buf.len() - take;
        (buf.iter().skip(start).cloned().collect(), pushed)
    }
}
