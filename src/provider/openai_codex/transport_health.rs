//! Tracks whether the Codex WebSocket transport should be bypassed.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Shared health state carried across stream restart attempts.
#[derive(Clone, Debug, Default)]
pub(super) struct TransportHealth(Arc<AtomicBool>);

impl TransportHealth {
    /// Return whether later attempts should use HTTP streaming.
    pub(super) fn requires_http(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    /// Disable WebSocket use after an interrupted attempt.
    pub(super) fn mark_interrupted(&self) {
        self.0.store(true, Ordering::Release);
    }
}
