//! RAII guard for one pending residency slot.

use super::state::ResidencyState;
use std::sync::Arc;

/// Pending capacity reservation that rolls back unless committed.
pub(in crate::tool::agent) struct Slot {
    state: Arc<ResidencyState>,
    active: bool,
}

impl Slot {
    pub(super) fn try_new(limit: usize) -> Option<Self> {
        Self::try_with(super::order::shared(), limit)
    }

    pub(super) fn try_with(state: Arc<ResidencyState>, limit: usize) -> Option<Self> {
        state.try_reserve(limit).then_some(Self {
            state,
            active: true,
        })
    }

    /// Convert this pending reservation into an LRU resident.
    pub(in crate::tool::agent) fn commit(mut self, agent_id: &str) {
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
