//! Pending-slot accounting and commit/rollback transitions.

use super::ResidencyState;

impl ResidencyState {
    pub(in crate::tool::agent::residency) fn try_reserve(&self, limit: usize) -> bool {
        let mut state = self.lock();
        if state.residents.len().saturating_add(state.pending) >= limit {
            return false;
        }
        state.pending += 1;
        true
    }

    pub(in crate::tool::agent::residency) fn commit(&self, agent_id: &str) {
        let mut state = self.lock();
        state.pending = state.pending.saturating_sub(1);
        state.residents.retain(|id| id != agent_id);
        state.residents.push_back(agent_id.into());
    }

    pub(in crate::tool::agent::residency) fn release(&self) {
        let mut state = self.lock();
        state.pending = state.pending.saturating_sub(1);
    }

    pub(in crate::tool::agent::residency) fn usage(&self) -> usize {
        let state = self.lock();
        state.residents.len().saturating_add(state.pending)
    }
}
