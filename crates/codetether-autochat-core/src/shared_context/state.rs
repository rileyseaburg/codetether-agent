//! Shared relay memory state operations.
use super::*;
impl SharedRelayContext {
    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
            && self.decisions.is_empty()
            && self.risks.is_empty()
            && self.next_actions.is_empty()
            && self.evidence.is_empty()
    }

    pub fn item_count(&self) -> usize {
        self.facts.len()
            + self.decisions.len()
            + self.risks.len()
            + self.next_actions.len()
            + self.evidence.len()
    }
}
