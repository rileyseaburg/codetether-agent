//! RAII guard for one pending residency slot.

use super::state::ResidencyState;
use std::sync::Arc;

/// Pending capacity reservation that rolls back unless committed.
///
/// # Examples
/// ```
/// let reservation=codetether_collaboration_state::residency::Slot::try_new(1);
/// drop(reservation);
/// ```
pub struct Slot {
    state: Arc<ResidencyState>,
    active: bool,
}

impl Slot {
    /// Reserve capacity against the process-wide residency ledger.
    ///
    /// Returns none when current residents and pending reservations reach the limit.
    pub fn try_new(limit: usize) -> Option<Self> {
        Self::try_with(super::order::shared(), limit)
    }

    pub(super) fn try_with(state: Arc<ResidencyState>, limit: usize) -> Option<Self> {
        state.try_reserve(limit).then_some(Self {
            state,
            active: true,
        })
    }

    /// Convert this pending reservation into an LRU resident.
    pub fn commit(mut self, agent_id: &str) {
        self.state.commit(agent_id);
        self.active = false;
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        if self.active {
            self.state.release();
        }
    }
}
