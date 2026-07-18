//! Resident LRU mutation and candidate selection.

use super::ResidencyState;

impl ResidencyState {
    pub(in crate::tool::agent::residency) fn touch(&self, agent_id: &str) {
        let mut state = self.lock();
        state.residents.retain(|id| id != agent_id);
        state.residents.push_back(agent_id.into());
    }

    pub(in crate::tool::agent::residency) fn forget(&self, agent_id: &str) {
        self.lock().residents.retain(|id| id != agent_id);
    }

    pub(in crate::tool::agent::residency) fn resident_count(&self) -> usize {
        self.lock().residents.len()
    }

    pub(in crate::tool::agent::residency) fn pop_candidate(
        &self,
        protected: Option<&str>,
    ) -> Option<String> {
        let mut state = self.lock();
        let count = state.residents.len();
        for _ in 0..count {
            let candidate = state.residents.pop_front()?;
            if protected == Some(candidate.as_str()) {
                state.residents.push_back(candidate);
                continue;
            }
            return Some(candidate);
        }
        None
    }
}
