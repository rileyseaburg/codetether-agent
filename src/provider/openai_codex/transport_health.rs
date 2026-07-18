//! Tracks whether the Codex WebSocket transport should be bypassed.

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard, PoisonError};

pub(super) const UNSCOPED: &str = "__codetether_unscoped__";
const MAX_TRACKED_SESSIONS: usize = 4_096;

/// Shared health state carried across stream restart attempts.
#[derive(Clone, Debug, Default)]
pub(super) struct TransportHealth(Arc<Mutex<HealthState>>);

#[derive(Debug, Default)]
struct HealthState {
    interrupted: HashSet<String>,
    insertion_order: VecDeque<String>,
}

impl TransportHealth {
    /// Return whether later attempts should use HTTP streaming.
    pub(super) fn requires_http(&self, session_id: &str) -> bool {
        self.state().interrupted.contains(session_id)
    }

    /// Disable WebSocket use after an interrupted attempt.
    pub(super) fn mark_interrupted(&self, session_id: &str) {
        let mut state = self.state();
        if !state.interrupted.insert(session_id.to_string()) {
            return;
        }
        state.insertion_order.push_back(session_id.to_string());
        if state.interrupted.len() > MAX_TRACKED_SESSIONS
            && let Some(evicted) = state.insertion_order.pop_front()
        {
            state.interrupted.remove(&evicted);
        }
    }

    fn state(&self) -> MutexGuard<'_, HealthState> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
#[path = "transport_health_tests.rs"]
mod tests;
